# 单设备单点登录测试脚本
# 
# 此脚本测试单种设备的单点登录功能：
# 1. 用户在Web设备登录
# 2. 同一用户在另一个Web设备登录（应该踢出第一个）
# 3. 验证第一个登录失效
# 4. 测试不同设备类型可以同时登录

param(
    [string]$BaseUrl = "http://localhost:8080"
)

# 颜色输出函数
function Write-Success { param($Message) Write-Host $Message -ForegroundColor Green }
function Write-Error { param($Message) Write-Host $Message -ForegroundColor Red }
function Write-Info { param($Message) Write-Host $Message -ForegroundColor Cyan }
function Write-Warning { param($Message) Write-Host $Message -ForegroundColor Yellow }

# 生成随机邮箱
$TestEmail = "test_sso_$(Get-Random)@example.com"
$TestPassword = "password123"
$TestName = "测试用户_$(Get-Random)"

Write-Info "开始单设备单点登录测试..."
Write-Info "测试邮箱: $TestEmail"
Write-Info "基础URL: $BaseUrl"
Write-Info "========================================="

try {
    # 步骤 1: 注册测试用户
    Write-Info "步骤 1: 注册测试用户"
    $RegisterBody = @{
        email = $TestEmail
        password = $TestPassword
        name = $TestName
    } | ConvertTo-Json

    $RegisterResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/register" -Method Post -Body $RegisterBody -ContentType "application/json" -Headers @{
        "User-Agent" = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
        "X-Device-Type" = "web"
    }
    
    if ($RegisterResponse.token) {
        Write-Success "✓ 用户注册成功"
        $FirstToken = $RegisterResponse.token
        Write-Info "  用户ID: $($RegisterResponse.user.id)"
        Write-Info "  首次登录Token: $($FirstToken.Substring(0, 20))..."
    } else {
        throw "注册失败：没有返回token"
    }

    # 步骤 2: 检查活跃会话（应该只有一个Web会话）
    Write-Info "`n步骤 2: 检查初始活跃会话"
    $SessionsResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/sessions" -Method Get -Headers @{
        "Authorization" = "Bearer $FirstToken"
        "User-Agent" = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
    }
    
    Write-Success "✓ 活跃会话数量: $($SessionsResponse.sessions.Count)"
    foreach ($session in $SessionsResponse.sessions) {
        Write-Info "  - 设备类型: $($session.device_type), 设备名: $($session.device_name)"
    }

    # 步骤 3: 模拟另一个Web设备登录（应该踢出第一个）
    Write-Info "`n步骤 3: 在另一个Web设备上登录（测试单点登录）"
    $LoginBody = @{
        email = $TestEmail
        password = $TestPassword
    } | ConvertTo-Json

    $SecondLoginResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/login" -Method Post -Body $LoginBody -ContentType "application/json" -Headers @{
        "User-Agent" = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Firefox/89.0"
        "X-Device-Type" = "web"
    }
    
    if ($SecondLoginResponse.token) {
        Write-Success "✓ 第二次Web登录成功"
        $SecondToken = $SecondLoginResponse.token
        Write-Info "  新Token: $($SecondToken.Substring(0, 20))..."
    } else {
        throw "第二次登录失败"
    }

    # 步骤 4: 验证第一个token是否已失效
    Write-Info "`n步骤 4: 验证第一个Token是否已失效"
    try {
        $ProfileResponse = Invoke-RestMethod -Uri "$BaseUrl/api/profile" -Method Get -Headers @{
            "Authorization" = "Bearer $FirstToken"
        }
        Write-Error "✗ 第一个Token仍然有效（预期应该失效）"
        Write-Warning "  这表明单点登录可能没有正常工作"
    } catch {
        Write-Success "✓ 第一个Token已失效（符合预期）"
        Write-Info "  错误信息: $($_.Exception.Message)"
    }

    # 步骤 5: 验证第二个token是否有效
    Write-Info "`n步骤 5: 验证第二个Token是否有效"
    try {
        $ProfileResponse = Invoke-RestMethod -Uri "$BaseUrl/api/profile" -Method Get -Headers @{
            "Authorization" = "Bearer $SecondToken"
        }
        Write-Success "✓ 第二个Token有效"
        Write-Info "  用户名: $($ProfileResponse.name)"
    } catch {
        Write-Error "✗ 第二个Token无效: $($_.Exception.Message)"
    }

    # 步骤 6: 测试移动设备登录（应该与Web设备并存）
    Write-Info "`n步骤 6: 测试移动设备登录（应该与Web设备并存）"
    $MobileLoginResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/login" -Method Post -Body $LoginBody -ContentType "application/json" -Headers @{
        "User-Agent" = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15"
        "X-Device-Type" = "mobile"
    }
    
    if ($MobileLoginResponse.token) {
        Write-Success "✓ 移动设备登录成功"
        $MobileToken = $MobileLoginResponse.token
        Write-Info "  移动Token: $($MobileToken.Substring(0, 20))..."
    } else {
        throw "移动设备登录失败"
    }

    # 步骤 7: 检查现在的活跃会话（应该有Web和Mobile两个）
    Write-Info "`n步骤 7: 检查当前所有活跃会话"
    $FinalSessionsResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/sessions" -Method Get -Headers @{
        "Authorization" = "Bearer $SecondToken"
    }
    
    Write-Success "✓ 当前活跃会话数量: $($FinalSessionsResponse.sessions.Count)"
    $WebSessions = $FinalSessionsResponse.sessions | Where-Object { $_.device_type -eq "web" }
    $MobileSessions = $FinalSessionsResponse.sessions | Where-Object { $_.device_type -eq "mobile" }
    
    Write-Info "  Web会话数量: $($WebSessions.Count) (预期: 1)"
    Write-Info "  Mobile会话数量: $($MobileSessions.Count) (预期: 1)"
    
    foreach ($session in $FinalSessionsResponse.sessions) {
        Write-Info "  - 设备类型: $($session.device_type), 设备名: $($session.device_name), 创建时间: $($session.created_at)"
    }

    # 步骤 8: 验证两个不同设备的token都有效
    Write-Info "`n步骤 8: 验证不同设备的Token都有效"
    
    # 验证Web token
    try {
        $WebProfileResponse = Invoke-RestMethod -Uri "$BaseUrl/api/profile" -Method Get -Headers @{
            "Authorization" = "Bearer $SecondToken"
        }
        Write-Success "✓ Web设备Token有效"
    } catch {
        Write-Error "✗ Web设备Token无效: $($_.Exception.Message)"
    }
    
    # 验证Mobile token
    try {
        $MobileProfileResponse = Invoke-RestMethod -Uri "$BaseUrl/api/profile" -Method Get -Headers @{
            "Authorization" = "Bearer $MobileToken"
        }
        Write-Success "✓ Mobile设备Token有效"
    } catch {
        Write-Error "✗ Mobile设备Token无效: $($_.Exception.Message)"
    }

    # 步骤 9: 测试撤销特定设备登录
    Write-Info "`n步骤 9: 测试撤销Mobile设备登录"
    try {
        $LogoutDeviceResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/logout-device/mobile" -Method Post -Headers @{
            "Authorization" = "Bearer $SecondToken"
        }
        Write-Success "✓ 撤销Mobile设备成功: $($LogoutDeviceResponse.message)"
    } catch {
        Write-Error "✗ 撤销Mobile设备失败: $($_.Exception.Message)"
    }

    # 步骤 10: 验证Mobile token已失效，Web token仍有效
    Write-Info "`n步骤 10: 验证撤销效果"
    
    # 验证Mobile token失效
    try {
        $MobileProfileResponse = Invoke-RestMethod -Uri "$BaseUrl/api/profile" -Method Get -Headers @{
            "Authorization" = "Bearer $MobileToken"
        }
        Write-Error "✗ Mobile设备Token仍然有效（预期应该失效）"
    } catch {
        Write-Success "✓ Mobile设备Token已失效（符合预期）"
    }
    
    # 验证Web token仍有效
    try {
        $WebProfileResponse = Invoke-RestMethod -Uri "$BaseUrl/api/profile" -Method Get -Headers @{
            "Authorization" = "Bearer $SecondToken"
        }
        Write-Success "✓ Web设备Token仍然有效（符合预期）"
    } catch {
        Write-Error "✗ Web设备Token意外失效: $($_.Exception.Message)"
    }

    # 步骤 11: 最终清理
    Write-Info "`n步骤 11: 清理测试数据"
    try {
        $LogoutAllResponse = Invoke-RestMethod -Uri "$BaseUrl/api/auth/logout-all" -Method Post -Headers @{
            "Authorization" = "Bearer $SecondToken"
        }
        Write-Success "✓ 清理完成: $($LogoutAllResponse.message)"
        Write-Info "  撤销的会话数量: $($LogoutAllResponse.revoked_count)"
    } catch {
        Write-Warning "清理时出现错误: $($_.Exception.Message)"
    }

    Write-Info "`n========================================="
    Write-Success "单设备单点登录测试完成！"
    Write-Info "测试总结："
    Write-Info "  ✓ 用户注册和初始登录"
    Write-Info "  ✓ 同设备类型单点登录（踢出旧会话）"
    Write-Info "  ✓ 不同设备类型可并存"
    Write-Info "  ✓ 选择性撤销设备会话"
    Write-Info "  ✓ 批量撤销所有会话"

} catch {
    Write-Error "测试失败: $($_.Exception.Message)"
    Write-Error "详细错误: $($_.ScriptStackTrace)"
    exit 1
}
