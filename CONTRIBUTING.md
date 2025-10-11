# 贡献指南

感谢你考虑为 sa-token-rust 做出贡献！

## 如何贡献

### 报告Bug

如果你发现了bug，请创建一个issue，并包含：

- 清晰的标题和描述
- 重现步骤
- 预期行为
- 实际行为
- 环境信息（操作系统、Rust版本等）
- 相关代码片段或错误信息

### 提出新功能

如果你有新功能的想法：

1. 先检查是否已有类似的issue或PR
2. 创建一个issue讨论你的想法
3. 等待维护者的反馈

### 提交代码

1. Fork本仓库
2. 创建你的特性分支：`git checkout -b feature/amazing-feature`
3. 编写代码并确保：
   - 代码风格符合Rust规范（运行 `cargo fmt`）
   - 没有编译警告（运行 `cargo clippy`）
   - 测试通过（运行 `cargo test`）
   - 添加了必要的文档
4. 提交改动：`git commit -m 'Add some amazing feature'`
5. 推送到分支：`git push origin feature/amazing-feature`
6. 开启Pull Request

## 代码规范

### Rust代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循Rust官方命名规范
- 为公共API编写文档注释

### 提交信息

使用清晰的提交信息：

```
feat: 添加新功能
fix: 修复bug
docs: 更新文档
style: 代码格式调整
refactor: 代码重构
test: 添加测试
chore: 构建/工具链相关
```

### 文档

- 所有公共API必须有文档注释
- 提供使用示例
- 更新相关的README和文档

## 开发流程

### 设置开发环境

```bash
# 克隆仓库
git clone https://github.com/your-username/sa-token-rust.git
cd sa-token-rust

# 构建项目
cargo build

# 运行测试
cargo test

# 运行格式化
cargo fmt

# 运行linter
cargo clippy
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --package sa-token-core

# 运行集成测试
cargo test --test integration_test
```

### 添加新的框架支持

1. 创建新的plugin crate：`sa-token-plugin-{framework}`
2. 实现 `SaRequest` 和 `SaResponse` trait
3. 实现框架的中间件机制
4. 添加测试和示例
5. 更新文档

### 添加新的存储后端

1. 创建新的storage crate：`sa-token-storage-{backend}`
2. 实现 `SaStorage` trait
3. 添加测试
4. 更新文档

## 行为准则

- 尊重所有贡献者
- 保持专业和礼貌
- 接受建设性的批评
- 关注对项目最有利的事情

## 问题？

如果你有任何问题，可以：

- 创建一个issue
- 在Discussions中讨论
- 发送邮件给维护者

再次感谢你的贡献！

