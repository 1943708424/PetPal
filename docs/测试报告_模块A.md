# 测试报告_模块A（window-shell）

## 1. 测试范围与环境

- 模块：`window-shell`（工作区边界钳制 `clamp_window_position` + Tauri 命令 `clamp_window_to_work_area` + Svelte 展示组件 `PetShell`）
- Rust 运行环境：`rustc 1.94.0` / `cargo 1.94.0`
- Node 运行环境：`node v24.12.0` / `npm 11.7.0`
- OS：Windows 10（10.0.19045）

## 2. 单元测试验证（独立运行）

### 2.1 Rust（集成/单元）

- 命令：`cargo test --all-features`
- 结果：通过
- 用例覆盖：
  - `src/lib.rs`：`greet_includes_name` 通过
  - `tests/window_layout.rs`：6/6 通过（溢出右/左上、宽/高大于工作区、零尺寸、非零工作区偏移）

### 2.2 前端（Vitest + 测试库）

- 命令：`npm run test`（Vitest）
- 结果：通过
- 测试文件：
  - `src/lib/window-layout.test.ts`：6 用例通过
  - `src/lib/components/PetShell.svelte.test.ts`：2 用例通过（合计 8）

结论：**单元测试通过率 100%**。

## 3. 覆盖率验证（Tarpulin）

- 命令：`cargo tarpaulin --all-features --engine Llvm --fail-under 90 --out Lcov --timeout 60`
- 结果：**不通过**
- 覆盖率结果：**64.00%**（低于 90% 门槛）
- 关键行覆盖：
  - `src/lib.rs`: 0/7
  - `src/main.rs`: 0/2
  - `src/window_layout.rs`: 16/16

## 4. 代码质量检查（Clippy）

- 命令：`cargo clippy --all-targets -- -D warnings`
- 结果：**通过**

## 5. 依赖漏洞扫描（cargo audit）

- 命令：`cargo audit`
- 结果：扫描完成
- 统计：`warning: 18 allowed warnings found`
- 内容要点：
  - 主要为 RustSec 的 “unmaintained / unsoundness” 类预警（如 gtk-rs GTK3 bindings no longer maintained，glib 的 unsoundness 等）
  - 本次扫描未看到需要“拒绝构建”的高危/致命漏洞结论（以 `cargo audit` 最终 exit code/拒绝项为准）

## 6. CI 门禁校验（GitHub Actions）

- 结论：**覆盖率门禁尚未能在 CI 上验证**
- 原因：最新的工作流运行在 `Rust — 单元测试（cargo test）` 阶段失败，失败点为 GTK/相关 native 依赖构建时的 `pkg-config exited with status code 1`（如 `gobject-sys` / `gdk-sys` / `gio-sys` / `glib-sys`），因此 `cargo tarpaulin` 步骤未执行。

## 7. 性能与稳定性（本阶段）

- 该模块核心为 O(1) 纯计算与轻量 UI 渲染；未进行 OS 级内存/CPU 采样。
- 单测执行耗时（参考）：
  - `cargo test`：约 6~14s（取决于是否需要首次编译/缓存）
  - `vitest`：约 3~5s（取决于缓存）

## 8. 验收结论

- 功能正确性（单元测试）：**通过**
- 覆盖率门禁（≥90%）：**不通过（本机 64%）**；CI 当前在构建阶段失败，覆盖率步骤未能执行
- 质量门禁（clippy 无警告）：**通过**

最终结论：**功能正确性通过，但覆盖率门禁与 CI 环境门禁未满足**；需要开发 Agent 提升覆盖率并修复 Ubuntu CI 依赖后再回归测试门禁。

## 9. 待修复清单（Bug/门禁）

1. 提升覆盖率至 ≥90%：当前 `src/lib.rs`、`src/main.rs` 在 tarpaulin 下为 0 覆盖，需要补齐测试或将不可测入口在 tarpaulin 下排除（与团队规则约定一致）
2. 修复 GitHub Actions 的 Ubuntu CI native 依赖：安装 GTK/WebKit 等必要依赖，使 `cargo test` 能在 CI 成功构建并执行后续 tarpaulin/clippy/audit 步骤