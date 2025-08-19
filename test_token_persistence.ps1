# Token 持久化功能测试脚本

$base_url = "http://localhost:3000"

Write-Host "开始测试 Token 持久化功能..." -ForegroundColor Green

# 测试用户注册
Write-Host "`n1. 测试用户注册" -ForegroundColor Yellow
$register_body = @{
    email = "test@example.com"
    password = "password123"
    name = "测试用户"
} | ConvertTo-Json

try {
    $register_response = Invoke-RestMethod -Uri "$base_url/api/auth/register" -Method POST -Body $register_body -ContentType "application/json"
    Write-Host "注册成功！" -ForegroundColor Green
    Write-Host "用户ID: $($register_response.user.id)"
    Write-Host "Token: $($register_response.token.Substring(0, 20))..."
    
    $token = $register_response.token
} catch {
    Write-Host "注册失败: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# 测试受保护的端点访问
Write-Host "`n2. 测试受保护端点访问" -ForegroundColor Yellow
$headers = @{
    "Authorization" = "Bearer $token"
}

try {
    $profile_response = Invoke-RestMethod -Uri "$base_url/api/profile" -Method GET -Headers $headers
    Write-Host "获取用户资料成功！" -ForegroundColor Green
    Write-Host "用户名: $($profile_response.name)"
    Write-Host "邮箱: $($profile_response.email)"
} catch {
    Write-Host "获取用户资料失败: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试退出登录
Write-Host "`n3. 测试退出登录" -ForegroundColor Yellow
try {
    $logout_response = Invoke-RestMethod -Uri "$base_url/api/auth/logout" -Method POST -Headers $headers
    Write-Host "退出登录成功！" -ForegroundColor Green
    Write-Host "消息: $($logout_response.message)"
} catch {
    Write-Host "退出登录失败: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试已撤销的token是否还能访问受保护端点
Write-Host "`n4. 测试已撤销token访问受保护端点" -ForegroundColor Yellow
try {
    $profile_response2 = Invoke-RestMethod -Uri "$base_url/api/profile" -Method GET -Headers $headers
    Write-Host "❌ 警告：已撤销的token仍然可以访问受保护端点！" -ForegroundColor Red
} catch {
    Write-Host "✅ 正确：已撤销的token无法访问受保护端点" -ForegroundColor Green
}

# 测试重新登录
Write-Host "`n5. 测试重新登录" -ForegroundColor Yellow
$login_body = @{
    email = "test@example.com"
    password = "password123"
} | ConvertTo-Json

try {
    $login_response = Invoke-RestMethod -Uri "$base_url/api/auth/login" -Method POST -Body $login_body -ContentType "application/json"
    Write-Host "重新登录成功！" -ForegroundColor Green
    $new_token = $login_response.token
    
    # 测试新token访问
    $new_headers = @{
        "Authorization" = "Bearer $new_token"
    }
    $profile_response3 = Invoke-RestMethod -Uri "$base_url/api/profile" -Method GET -Headers $new_headers
    Write-Host "新token访问受保护端点成功！" -ForegroundColor Green
} catch {
    Write-Host "重新登录失败: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试退出所有设备
Write-Host "`n6. 测试退出所有设备" -ForegroundColor Yellow
try {
    $logout_all_response = Invoke-RestMethod -Uri "$base_url/api/auth/logout-all" -Method POST -Headers $new_headers
    Write-Host "退出所有设备成功！" -ForegroundColor Green
    Write-Host "消息: $($logout_all_response.message)"
    Write-Host "撤销的token数量: $($logout_all_response.revoked_count)"
} catch {
    Write-Host "退出所有设备失败: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n✅ Token 持久化功能测试完成！" -ForegroundColor Green
Write-Host "请确保服务器正在运行：npm run dev 或 cargo run" -ForegroundColor Cyan
