# Rust 后端骨架

一个使用 Axum + SQLx(PostgreSQL) + JWT + Argon2 + Tracing 的现代 Rust 后端应用程序。

## 功能特性

- **Web 框架**: Axum - 高性能异步 web 框架
- **数据库**: PostgreSQL + SQLx - 类型安全的数据库访问
- **认证**: JWT (JSON Web Tokens) 用于无状态认证
- **密码加密**: Argon2 - 现代密码散列算法
- **日志**: Tracing - 结构化日志和分布式跟踪
- **中间件**: CORS、认证中间件
- **错误处理**: 自定义错误类型和统一错误响应

## 项目结构

```
src/
├── config.rs           # 应用配置
├── db.rs              # 数据库连接池
├── error.rs           # 错误处理
├── lib.rs             # 库入口
├── main.rs            # 应用入口
├── routes.rs          # 路由定义
├── handlers/          # 请求处理器
│   ├── auth.rs        # 认证处理器
│   ├── user.rs        # 用户处理器
│   └── mod.rs
├── middleware/        # 中间件
│   ├── auth.rs        # 认证中间件
│   └── mod.rs
├── models/            # 数据模型
│   ├── user.rs        # 用户模型
│   └── mod.rs
├── services/          # 业务逻辑
│   ├── user_service.rs # 用户服务
│   └── mod.rs
└── utils/             # 工具函数库
    ├── auth.rs        # JWT 认证工具
    ├── password.rs    # 密码加密工具
    ├── time.rs        # 时间处理工具
    ├── string.rs      # 字符串工具
    ├── number.rs      # 数字工具
    ├── collection.rs  # 集合工具
    ├── crypto.rs      # 加密工具
    ├── convert.rs     # 类型转换工具
    ├── format.rs      # 格式化工具
    └── mod.rs
```

## 快速开始

### 1. 环境要求

- Rust 1.70+
- PostgreSQL 12+

### 2. 设置数据库

```bash
# 创建数据库
createdb hello_rust

# 或使用 psql
psql -U postgres -c "CREATE DATABASE hello_rust;"
```

### 3. 配置环境变量

```bash
# 复制环境变量示例文件
cp .env.example .env

# 编辑 .env 文件，设置正确的数据库连接字符串
# DATABASE_URL=postgresql://postgres:password@localhost:5432/hello_rust
```

### 4. 运行应用

```bash
# 安装依赖并运行
cargo run
```

应用将在 `http://localhost:3000` 启动。

## API 端点

### 认证

#### 注册用户
```http
POST /api/auth/register
Content-Type: application/json

{
    "email": "user@example.com",
    "password": "password123",
    "name": "用户名"
}
```

#### 用户登录
```http
POST /api/auth/login
Content-Type: application/json

{
    "email": "user@example.com",
    "password": "password123"
}
```

### 用户管理 (需要认证)

#### 获取当前用户信息
```http
GET /api/profile
Authorization: Bearer <jwt_token>
```

#### 获取所有用户
```http
GET /api/users
Authorization: Bearer <jwt_token>
```

### 健康检查

```http
GET /health
```

## 开发

### 数据库迁移

数据库迁移文件位于 `migrations/` 目录，应用启动时会自动运行。

### 测试 API

可以使用 curl 或 Postman 测试 API：

```bash
# 注册用户
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123","name":"测试用户"}'

# 登录获取 token
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'

# 使用 token 访问受保护的端点
curl -X GET http://localhost:3000/api/profile \
  -H "Authorization: Bearer <your_jwt_token>"
```

## 安全注意事项

- 在生产环境中，确保更改 `JWT_SECRET` 为强密码
- 使用 HTTPS 来保护敏感数据传输
- 定期更新依赖项以修复安全漏洞
- 考虑实施速率限制和其他安全中间件

## 工具库功能

项目内置了一套完整的工具库，包含以下模块：

### 🕐 时间工具 (TimeUtils)
- 时间格式化和解析
- 时间运算（加减、比较）
- 相对时间显示
- 时间范围操作
- **时区转换和管理**
  - UTC 与各时区间转换
  - 世界时钟功能
  - 时差计算
  - 夏令时检测
  - 时区偏移查询
  - 按偏移查找时区

### 🔤 字符串工具 (StringUtils)
- 命名转换（驼峰、下划线）
- 字符串验证（邮箱、手机号等）
- 文本处理（截断、填充、反转）
- 随机字符串生成
- 字符串相似度计算

### 🔢 数字工具 (NumberUtils)
- 数学运算（质数、阶乘、斐波那契）
- 统计计算（平均值、中位数、标准差）
- 进制转换
- 随机数生成
- 数字格式化

### 📦 集合工具 (CollectionUtils)
- 数组操作（去重、分块、排序）
- 集合运算（交集、并集、差集）
- 数据分组和聚合
- 频率统计
- 搜索和过滤

### 🔐 加密工具 (CryptoUtils)
- Base64/十六进制编码
- 密码生成和强度检查
- 简单加密算法（凯撒、异或）
- UUID 生成
- 哈希计算

### 🔄 类型转换工具 (ConvertUtils)
- 安全类型转换
- JSON 处理
- URL 编解码
- CSV 处理
- 单位转换（温度、长度、重量等）

### 🎨 格式化工具 (FormatUtils)
- 货币和数字格式化
- 文件大小格式化
- 时间持续时间格式化
- 数据脱敏
- 表格和进度条生成

### 使用示例

```rust
use hello_rust::utils::*;

// 时间工具
let now = TimeUtils::now_utc();
let formatted = TimeUtils::format_default(&now);

// 时区转换
let beijing_time = TimeUtils::to_timezone(&now, Asia::Shanghai);
let world_clock = TimeUtils::world_clock(&[("北京", Asia::Shanghai), ("纽约", America::New_York)]);

// 字符串工具
let snake_case = StringUtils::camel_to_snake("camelCase");
let is_email = StringUtils::is_valid_email("test@example.com");

// 数字工具
let is_prime = NumberUtils::is_prime(17);
let average = NumberUtils::average(&[1.0, 2.0, 3.0]);

// 集合工具
let unique_items = CollectionUtils::unique(&[1, 2, 2, 3]);
let intersection = CollectionUtils::intersection(&vec1, &vec2);

// 加密工具
let encoded = CryptoUtils::base64_encode(data.as_bytes());
let password = CryptoUtils::generate_secure_password(12);

// 格式化工具
let currency = FormatUtils::format_currency(1234.56, "$", 2);
let progress = FormatUtils::format_progress_bar(75, 100, 20, '█', '░');
```

运行工具演示：
```bash
# 基础工具演示
cargo run --example utils_demo

# 时区工具演示
cargo run --example timezone_demo
```

## 扩展建议

- 添加用户角色和权限系统
- 实现密码重置功能
- 添加 API 版本控制
- 实现数据验证和清理
- 添加 API 文档 (OpenAPI/Swagger)
- 实现缓存策略 (Redis)
- 添加监控和指标收集
- 扩展工具库（图片处理、网络请求等）
