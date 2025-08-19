# API 测试脚本
$baseUrl = "http://localhost:3000"

Write-Host "测试 API 端点..." -ForegroundColor Green

# 测试健康检查
Write-Host "`n1. 测试健康检查..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
    Write-Host "健康检查: $response" -ForegroundColor Green
} catch {
    Write-Host "健康检查失败: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试用户注册
Write-Host "`n2. 测试用户注册..." -ForegroundColor Yellow
$registerData = @{
    email = "test@example.com"
    password = "password123"
    name = "测试用户"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/auth/register" -Method POST -Body $registerData -ContentType "application/json"
    Write-Host "注册成功!" -ForegroundColor Green
    $token = $response.token
    Write-Host "JWT Token: $($token.Substring(0, 20))..." -ForegroundColor Cyan
} catch {
    Write-Host "注册失败: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试用户登录
Write-Host "`n3. 测试用户登录..." -ForegroundColor Yellow
$loginData = @{
    email = "test@example.com"
    password = "password123"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/auth/login" -Method POST -Body $loginData -ContentType "application/json"
    Write-Host "登录成功!" -ForegroundColor Green
    $token = $response.token
    Write-Host "JWT Token: $($token.Substring(0, 20))..." -ForegroundColor Cyan
} catch {
    Write-Host "登录失败: $($_.Exception.Message)" -ForegroundColor Red
    return
}

# 测试获取用户信息
Write-Host "`n4. 测试获取用户信息..." -ForegroundColor Yellow
$headers = @{
    Authorization = "Bearer $token"
}

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/profile" -Method GET -Headers $headers
    Write-Host "用户信息获取成功!" -ForegroundColor Green
    Write-Host "用户ID: $($response.id)" -ForegroundColor Cyan
    Write-Host "邮箱: $($response.email)" -ForegroundColor Cyan
    Write-Host "姓名: $($response.name)" -ForegroundColor Cyan
} catch {
    Write-Host "获取用户信息失败: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试获取所有用户
Write-Host "`n5. 测试获取所有用户..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/users" -Method GET -Headers $headers
    Write-Host "获取所有用户成功! 用户数量: $($response.Count)" -ForegroundColor Green
} catch {
    Write-Host "获取所有用户失败: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`nAPI 测试完成!" -ForegroundColor Green
