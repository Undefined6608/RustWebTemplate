# 开发环境启动脚本
Write-Host "启动 Rust 后端开发环境..." -ForegroundColor Green

# 检查 Docker 是否正在运行
if (Get-Process docker -ErrorAction SilentlyContinue) {
    Write-Host "Docker 正在运行，启动 PostgreSQL..." -ForegroundColor Yellow
    docker-compose up -d postgres
    Start-Sleep 3
} else {
    Write-Host "警告: Docker 未运行，请确保 PostgreSQL 已启动" -ForegroundColor Red
}

# 检查数据库连接
Write-Host "检查数据库连接..." -ForegroundColor Yellow

# 设置环境变量
$env:RUST_LOG = "hello_rust=debug,tower_http=debug,sqlx=info"

# 启动应用
Write-Host "启动应用..." -ForegroundColor Green
cargo run
