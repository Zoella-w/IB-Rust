# My Crate

[![CI Status](https://github.com/yourusername/my-lib-crate/workflows/CI/badge.svg)](https://github.com/yourusername/my-lib-crate/actions)
[![Crates.io](https://img.shields.io/crates/v/my-lib-crate.svg)](https://crates.io/crates/my-lib-crate)
[![Documentation](https://docs.rs/my-lib-crate/badge.svg)](https://docs.rs/my-lib-crate)

一个演示 Rust 项目规范的库 crate，包含：
- 单元测试
- 集成测试
- 文档测试
- 基准测试
- 示例

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
my-lib-crate = "0.1.0"
```

## 功能

### 基础数学函数

```rust
use my_crate::math;

let sum = math::add(2, 3); // 5
let product = math::multiply(4, 5); // 20
let fib = math::fibonacci(10); // 55
```

## 运行测试

```bash
# 运行所有测试（单元、集成、文档测试）
cargo test --all-features
# 运行基准测试（需要nightly）
cargo +nightly bench
# 运行示例
cargo run --example basic_usage
# 运行 CLI 工具
cargo run --bin cli --features cli -- add 5 7
```

## 文档相关命令

```bash
# 生成 HTML 文档并在浏览器中打开
cargo doc --open
# 检查文档是否符合标准
cargo doc --no-deps
# 运行所有文档测试
cargo test --doc
# 特定模块的文档测试
cargo test --doc -- my_crate::math
# 检查文档覆盖率
cargo tarpaulin --doc --ignore-tests -o Html --output-dir doc-coverage
open doc-coverage/index.html
```

## 检查发布包

```bash
# 创建发布包（不实际发布）
cargo package
# 列出发布包内容
tar -ztvf target/package/my_crate-*.crate | less
# 验证包结构
cargo verify-project
# 检查许可证信息
cargo license --json | jq .
# 检查包依赖关系
cargo tree
# 检查包大小
du -sh target/package/my_crate-*.crate
```

## 登录 crates.io

```bash
# 登录（替换 YOUR_API_TOKEN）
cargo login --registry crates-io YOUR_API_TOKEN
# 登出（清除凭据）
cargo logout
# 安全登录（避免在历史中保留token）
read -s TOKEN
cargo login $TOKEN
```

## 手动发布

```bash
# 完整发布流程
cargo publish --verbose --registry crates-io 
# 无确认发布（适合自动化）
cargo publish --no-verify --allow-dirty --registry crates-io 
# 仅发布特定包（工作区中）
cargo publish -p my_crate --registry crates-io 
# 模拟发布
cargo publish --dry-run --registry crates-io 
# 发布带有特定功能
cargo publish --features cli --registry crates-io 
```

## 将代码推送到 main 分支触发 CI/CD 流程

```bash
# 更新版本号后提交并推送
git add Cargo.toml
git commit -m "Release v0.1.1"
git push origin main
# 查看 GitHub Actions 工作流状态
open https://github.com/yourusername/my_crate/actions
# 监视发布过程日志
gh run watch --repo yourusername/my_crate $(gh run list --repo yourusername/my_crate -L 1 --json databaseId -q '.[0].databaseId')
# 验证 crates.io 发布状态
open https://crates.io/crates/my_crate
```

## 贡献指南

欢迎提交 issue 和 PR！