# Diskmap

[English](./README.md) | 简体中文

Diskmap 是一款受 KDE 应用程序 [Filelight](https://github.com/KDE/filelight) 启发的终端磁盘空间可视化工具。它提供跨平台的磁盘占用可视化方案，帮助您更高效地管理存储空间。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-v1.90%2B-orange.svg)](https://www.rust-lang.org/)

## 🚀 项目介绍

Diskmap 旨在将 Filelight 直观的可视化体验带入终端。无论您是在寻找庞大的日志文件，还是仅仅对文件夹结构感到好奇，Diskmap 都能为您的存储空间提供清晰且可交互的地图。

> [!NOTE]
> 这是我的第一个开源项目！我开发 Diskmap 是为了在磨练 Rust 开发技能的同时回馈社区。非常欢迎您的反馈和贡献。

## 📦 安装指南

本项目目前处于早期开发阶段，尚未发布正式版本。

### 源码编译
如果您想尝试在本地构建（待核心逻辑完成后）：
```bash
git clone https://github.com/Mr-Octopus-2020/diskmap.git
cd diskmap
cargo build --release
```

## ✨ 功能特性

- [ ] **径向可视化**：通过美观的终端图表查看磁盘占用。
- [ ] **交互式探索**：直接在可视化界面中切换和浏览文件夹。
- [ ] **矩形树图 (Treemap)**：提供另一种文件夹层级视角的展示方式。
- [ ] **多语言支持**：完整的本地化界面支持。

## 🤝 参与贡献

贡献是开源社区成为一个学习、启发和创造的绝佳场所的动力。您的每一份贡献都**备受珍视**。

请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 获取贡献指南。

## ⚖️ 开源协议

本项目基于 MIT 协议进行分发。详情请参见 `LICENSE` 文件。

## 🙏 致谢

* [Filelight](https://github.com/KDE/filelight) - 项目灵感的原点。
* [Rust 社区](https://www.rust-lang.org/community) - 提供了极棒的资源与支持。
