# bevy-shuffle-seed

Bevy 0.20.1 `ScheduleBuildSettings.shuffle_seed` 的单线程复现实验。

`shuffle_seed` 用一个种子打乱 schedule 里没有顺序约束的系统，用来暴露隐藏的执行顺序依赖。本仓库用几个最小 crate 验证三件事：它要哪个 Cargo feature 才存在，同一种子是否稳定复现，以及补上 `after` 约束后问题是否消失。

## 范围

- 版本：Bevy / `bevy_ecs` **0.20.1**（`Cargo.toml` 里写死 `=0.20.1`，每个 crate 自带 `Cargo.lock`）。
- 需要 Cargo feature **`debug`**。不开 `debug` 时字段不存在，编译报 `E0560 struct ScheduleBuildSettings has no field named shuffle_seed`。
- 执行器显式设为 **`SingleThreadedExecutor`**（单线程）。每个进程只构建并运行一次 schedule；`auto_insert_apply_deferred` 保持默认，不含 Commands。
- 样本有限：扫描 73 个取值，即 `none`、`0..63`，加 8 个官方单测种子 `100000000 100000001 100000003 100000004 100000020 100000030 100000063 100000080`。
- 结果不保证在其他 Bevy 版本、其他执行器或其他配置下复现。
- 未测：MultiThreadedExecutor；`auto_insert_apply_deferred=false` / Commands 同步点；App / DefaultPlugins / 多 schedule；ambiguity detection。
- 工具链：rustc / cargo **1.99.1**（bevy_ecs 0.20.1 的 MSRV 是 1.95.0）。

## 目录

| 路径 | 内容 |
|---|---|
| `crates/e1-ecs-nodebug` | `bevy_ecs` 默认 features，无 `debug`，访问 `shuffle_seed`，预期编译失败 |
| `crates/e2-ecs-debug` | `bevy_ecs` + `debug`，实验二进制（场景 A / B / C） |
| `crates/e3-bevy-nodebug` | `bevy`（default-features=false），无 `debug`，预期编译失败 |
| `crates/e4-bevy-debug` | `bevy`（default-features=false）+ `debug`，预期编译通过 |
| `crates/e5-ecs-debug-fixed` | `bevy_ecs` + `debug`，场景 B（对照）与 F（补 `consume.after(produce)`） |
| `run-step.sh` | 包装器：记录时间、工具链、命令和退出码，输出写入 `logs/<名字>.log` |
| `run-experiments.sh` / `run-e5.sh` | 重复运行与种子扫描 |
| `analyze.py` / `analyze-e5.py` | 读 `logs/` 里的运行日志，输出判定 |
| `results.json` | 原始实验的结论摘要 |

5 个 crate 相互独立，不组成 Cargo workspace，避免 feature 被统一。`logs/` 和 `target/` 由脚本生成，不入库。

## 复现

前提：已装 rustup、Python 3、bash。以下命令都在仓库根目录执行。`run-step.sh` 只对单个进程设置 `RUSTUP_TOOLCHAIN=1.99.1`，不改 rustup 全局默认；构建目录是仓库根下的 `target/`。

如果机器上原本没有默认工具链，`rustup toolchain install` 会把 1.99.1 自动设为默认；不想保留时，跑完执行 `rustup default none` 还原。

e1 和 e3 预期退出码 101（编译失败）。逐条执行时不要因为这个退出码停掉后面的命令。

```bash
git clone https://github.com/foxzool/bevy-shuffle-seed
cd bevy-shuffle-seed
rustup toolchain install 1.99.1 --profile minimal

# F1：feature 门控
bash run-step.sh 01-e1-ecs-nodebug-build      crates/e1-ecs-nodebug  cargo build -j 2          # 预期 exit 101，E0560
bash run-step.sh 02-e2-ecs-debug-build-dev    crates/e2-ecs-debug    cargo build -j 2
bash run-step.sh 03-e2-ecs-debug-build-release crates/e2-ecs-debug   cargo build --release -j 2
bash run-step.sh 04-e3-bevy-nodebug-check     crates/e3-bevy-nodebug cargo check -j 2          # 预期 exit 101，E0560
bash run-step.sh 05-e4-bevy-debug-check       crates/e4-bevy-debug   cargo check -j 2

# 可选：看 debug feature 的传递链
(cd crates/e4-bevy-debug && RUSTUP_TOOLCHAIN=1.99.1 cargo tree -e features -i bevy_ecs)  # bevy → bevy_internal → bevy_ecs
(cd crates/e3-bevy-nodebug && RUSTUP_TOOLCHAIN=1.99.1 cargo tree -e features -i rand)   # 无 debug：没有 rand

# 场景 A / B / C：重复运行 + 73 个取值扫描
bash run-step.sh 10-run-experiments . bash ./run-experiments.sh
python3 analyze.py | tee logs/11-analysis.log

# E5：补约束校验（--offline 依赖上面 e2 已下载的 crates.io 缓存，所以要在 e2 构建之后跑）
bash run-step.sh 20-e5-ecs-debug-fixed-build-release crates/e5-ecs-debug-fixed cargo build --release --offline -j 2
bash run-step.sh 21-run-e5 . bash ./run-e5.sh
python3 analyze-e5.py | tee logs/22-analysis-e5.log
```

## 场景

- **A**：与 bevy_ecs 官方单测 `schedule_builds_randomly_with_shuffler` 同构的图（0 → 10,11；10 → 20,21），所有系统都写 `ResMut<Counters>`。
- **B**：隐藏依赖。`produce` 写 `Value=42`，`consume` 读 `Value`，两者之间不加 before/after；另有 2 个噪声系统。
- **C**：显式约束 `(c1,c2,c3).chain()`、`x.before(y)`、`z.after(y)`，外加 4 个自由系统 f1..f4。
- **F**（E5）：B 的同一组系统，补上 `consume.after(produce)`。

## 结果（原始实验实测，rustc 1.99.1）

| # | 结论 | 判定 |
|---|---|---|
| ① | 不开 `debug`：`bevy_ecs` 和 `bevy` 都报 E0560。开 `debug` 后 dev 和 release 都能编译，说明门控是 feature，不是 `debug_assertions`。`bevy "debug"` → `bevy_internal "debug"` → `bevy_ecs "debug"`；只有开 debug 的依赖树里有 `rand` | PASS |
| ② | 6 个种子（none,1,7,42,100000001,100000030）× 5 个进程 × 3 个场景，18/18 组输出一致（`REPEAT_ALL_IDENTICAL True`）。场景 A 的 8 个官方种子和 None 基线都与官方单测断言一致 | PASS |
| ③ | 场景 B：None 时 `produce` 先跑（读到 42，bug 被掩盖）；72 个非 None 种子里 37 个让 `consume` 先跑（读到 0），35 个读到 42 | PASS |
| ④ | 场景 C：73 个取值得到 73 种不同顺序，约束违反 0 次 | PASS |
| ⑤ | E5：重复 12/12 组一致；B 对照复现 ③；F 场景 73/73 读到 42，`consume` 先于 `produce` 0 次，共 12 种不同顺序 | PASS |

## 许可

MIT OR Apache-2.0，与 Bevy 相同。见 `LICENSE-MIT`、`LICENSE-APACHE`。
