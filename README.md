# 📝 Demeter TodoList

一个现代化、简约的任务管理应用，采用前后端分离架构。

## ✨ 特性

- 🎨 **现代化UI设计** - 简约美观的用户界面
- 📱 **响应式布局** - 完美适配各种设备
- 🎯 **任务管理** - 创建、编辑、删除和完成任务
- 😀 **Emoji支持** - 为每个任务设置个性化emoji图标
- 📊 **活动热力图** - GitHub风格的任务活动可视化
- 🔐 **用户认证** - 简单的用户名密码登录系统
- ⚡ **实时更新** - 流畅的用户体验

## 🛠️ 技术栈

### 前端
- **Vue 3** - 渐进式JavaScript框架
- **TypeScript** - 类型安全的JavaScript
- **Vite** - 快速的构建工具
- **Pinia** - Vue状态管理
- **Vue Router** - 路由管理
- **Axios** - HTTP客户端

### 后端
- **Rust** - 系统编程语言
- **Axum** - 异步web框架
- **SQLx** - 异步SQL工具包
- **SQLite** - 轻量级数据库
- **bcrypt** - 密码哈希
- **Tokio** - 异步运行时

## 📁 项目结构

```
Demeter/
├── frontend/          # Vue3前端应用
│   ├── src/
│   │   ├── views/     # 页面组件
│   │   ├── stores/    # Pinia状态管理
│   │   ├── router/    # 路由配置
│   │   └── assets/    # 静态资源
│   └── package.json
├── backend/           # Rust后端服务
│   ├── src/
│   │   ├── main.rs    # 主程序入口
│   │   ├── models.rs  # 数据模型
│   │   ├── handlers.rs # 请求处理器
│   │   └── database.rs # 数据库操作
│   └── Cargo.toml
└── README.md
```

## 🚀 快速开始

### 前置要求

- Node.js (推荐 v18+)
- Rust (推荐最新稳定版)
- npm 或 yarn

### 安装和运行

#### 1. 克隆项目
```bash
git clone <repository-url>
cd Demeter
```

#### 2. 启动后端服务
```bash
cd backend
cargo run
```
后端服务将在 `http://localhost:3001` 启动（可在config.toml中配置）

#### 3. 启动前端应用
```bash
cd frontend
npm install
npm run dev
```
前端应用将在 `http://localhost:5173` 启动

### 默认账户
- 用户名: `guest`
- 密码: `password`

## 📱 功能页面

### 🔐 登录页面
- 简洁的登录界面
- 用户名密码验证
- 响应式设计

### 📝 任务列表页面
- 查看当前任务
- 创建新任务
- 选择emoji图标
- 标记完成/未完成
- 删除任务
- 卡片式布局with动画效果

### 📊 历史记录页面
- GitHub风格的活动热力图
- 365天的任务活动记录
- 点击查看具体日期的任务详情
- 统计数据展示

## 🎨 设计特色

- **渐变背景** - 美观的渐变色彩搭配
- **卡片设计** - 现代化的卡片布局
- **动画效果** - 流畅的过渡动画
- **悬浮效果** - 交互式的悬浮反馈
- **响应式** - 完美适配移动端

## ⚙️ 配置文件

后端使用 `backend/config.toml` 配置文件管理服务器设置：

```toml
[server]
host = "0.0.0.0"              # 服务器监听地址
port = 3001                    # 服务器端口
cors_origin = "http://localhost:5173"  # CORS允许的源

[database]
url = "sqlite:./todolist.db"   # 数据库连接URL
create_if_missing = true       # 自动创建数据库文件

[logging]
level = "info"                 # 日志级别
```

## 🔧 开发

### 前端开发
```bash
cd frontend
npm run dev    # 开发模式
npm run build  # 构建生产版本
```

### 后端开发
```bash
cd backend
cargo run      # 运行开发服务器
cargo build    # 构建项目
cargo test     # 运行测试
```

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 联系

如有问题或建议，请通过 Issue 联系我们。
