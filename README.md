# 个人简历网站

使用 Rust + Rocket + Diesel + Tera 开发的现代化个人简历展示网站。

## 功能特性

- 📱 **响应式设计** - 完美适配桌面端和移动端
- 🎨 **现代化UI** - 使用现代设计语言和动画效果
- 🗄️ **数据库驱动** - 使用 SQLite 数据库存储简历数据
- 🚀 **高性能** - 基于 Rust 和 Rocket 框架的高性能 Web 应用
- 📄 **多页面展示** - 分页展示技能、经验、项目等信息
- 🔍 **SEO友好** - 服务端渲染，搜索引擎友好

## 技术栈

- **后端框架**: Rocket 0.5
- **ORM**: Diesel 2.1
- **模板引擎**: Tera
- **数据库**: SQLite
- **前端**: HTML5 + CSS3 + JavaScript (ES6+)
- **图标**: Font Awesome 6
- **字体**: Inter (Google Fonts)

## 项目结构

```
resume_app/
├── src/
│   ├── main.rs          # 应用程序入口
│   ├── models.rs        # 数据模型定义
│   ├── schema.rs        # 数据库 schema
│   ├── database.rs      # 数据库连接配置
│   ├── repository.rs    # 数据访问层
│   └── routes.rs        # 路由处理器
├── templates/           # Tera 模板文件
│   ├── base.html.tera   # 基础模板
│   ├── index.html.tera  # 首页模板
│   ├── skills.html.tera # 技能页面模板
│   ├── experience.html.tera # 经验页面模板
│   └── projects.html.tera   # 项目页面模板
├── static/              # 静态资源
│   ├── css/
│   │   └── style.css    # 主样式文件
│   └── js/
│       └── script.js    # JavaScript 脚本
├── migrations/          # 数据库迁移文件
├── Cargo.toml          # Rust 依赖配置
├── Rocket.toml         # Rocket 配置
└── diesel.toml         # Diesel 配置
```

## 安装和运行

### 前置要求

1. 安装 Rust (推荐使用 rustup)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装 Diesel CLI
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

### 运行步骤

1. 克隆项目
```bash
git clone <repository-url>
cd resume_app
```

2. 设置数据库
```bash
# 运行数据库迁移（首次运行时会自动创建数据库）
diesel migration run
```

3. 运行应用程序
```bash
cargo run
```

4. 访问应用程序
打开浏览器访问 `http://localhost:8000`

## 页面说明

- **首页** (`/`) - 完整的简历概览，包含个人信息、技能概览、工作经验、项目经验和教育背景
- **技能详情** (`/skills`) - 详细的技能展示，按分类显示技能熟练度
- **工作经验** (`/experience`) - 详细的工作经历和成就
- **项目经验** (`/projects`) - 项目作品展示，包含技术栈和链接
- **API接口** (`/api/resume`) - JSON 格式的简历数据接口

## 自定义数据

项目包含示例数据，您可以通过以下方式自定义：

1. **修改数据库数据** - 直接编辑 SQLite 数据库文件
2. **修改迁移文件** - 编辑 `migrations/*/up.sql` 文件中的示例数据
3. **重新运行迁移** - 删除数据库文件后重新运行 `diesel migration run`

## 部署

### 生产环境构建

```bash
cargo build --release
```

### Docker 部署

创建 Dockerfile:
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/resume_app .
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static
COPY --from=builder /app/Rocket.toml .
EXPOSE 8000
CMD ["./resume_app"]
```

## 开发说明

### 添加新功能

1. **数据模型** - 在 `src/models.rs` 中定义新的数据结构
2. **数据库 Schema** - 在 `src/schema.rs` 中添加表定义
3. **数据访问** - 在 `src/repository.rs` 中添加数据库操作
4. **路由处理** - 在 `src/routes.rs` 中添加新的路由
5. **模板** - 在 `templates/` 目录中创建新的模板文件

### 样式自定义

主要的 CSS 变量定义在 `static/css/style.css` 的 `:root` 选择器中，您可以轻松自定义颜色主题：

```css
:root {
    --primary-color: #2563eb;    /* 主色调 */
    --secondary-color: #64748b;  /* 次要颜色 */
    --accent-color: #f59e0b;     /* 强调色 */
    /* ... 更多变量 */
}
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
