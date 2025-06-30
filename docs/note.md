# 项目结构
rusite-front-ending/
├── Cargo.toml             # Rust项目配置文件
├── Trunk.toml             # Trunk构建工具配置文件
├── index.html             # 主HTML入口文件
├── package.json           # npm管理CSS工具
├── tailwind.config.js     # Tailwind CSS配置文件
├── README.md
├── .gitignore
├── public/                # 静态资源文件（图片、字体等）
│   ├── favicon.ico
│   ├── images/
│   └── fonts/
├── assets/                # 需要处理的静态资源（如CSS、SCSS）
│   ├── input.css          # Tailwind的入口CSS文件
│   └── ...               
├── src/
│   ├── main.rs            # 应用主入口
│   ├── lib.rs             # 库入口
│   ├── app.rs             # 根组件
│   ├── components/        # 可复用的UI组件
│   │   ├── layout/
│   │   │   ├── header.rs
│   │   │   ├── footer.rs
│   │   │   └── mod.rs
│   │   ├── ui/
│   │   │   ├── button.rs
│   │   │   ├── card.rs
│   │   │   └── mod.rs
│   │   └── mod.rs                # 导出所有组件
│   ├── pages/             # 页面组件
│   │   ├── home.rs        # 首页
│   │   ├── blog/
│   │   │   ├── index.rs   # 博客列表页
│   │   │   ├── [id].rs    # 博客详情页（动态路由）
│   │   │   └── mod.rs
│   │   ├── about.rs       # 关于页面
│   │   ├── admin/         # 管理后台相关页面
│   │   │   ├── login.rs
│   │   │   ├── dashboard.rs
│   │   │   └── mod.rs
│   │   └── mod.rs         # 导出所有页面
│   ├── api/               # 网络请求相关
│   │   ├── mod.rs         # 导出API模块
│   │   ├── blog.rs        # 博客相关的API请求函数
│   │   ├── auth.rs        # 认证相关的API请求函数
│   │   └── client.rs      # HTTP客户端封装（使用reqwest或leptos提供的Resource）
│   ├── models/            # 数据模型
│   │   ├── mod.rs
│   │   ├── blog.rs        # 博客文章模型
│   │   ├── user.rs        # 用户模型
│   │   └── response.rs    # 后端API响应模型
│   ├── state.rs           # 应用状态管理
│   ├── utils.rs           # 工具函数
│   └── routes.rs          # 路由配置
└── .env                   # 环境变量

# Routers

/: Home of My Site
/blog: Blog, list of articles
/blog/: id: Article Page
/about: About
/admin: Admin To Me Push or modify articles 
/chat: Chat with my friends
/chat/: id: Chat group
/user?id=: User Profile