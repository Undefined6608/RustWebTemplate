/*!
 * 设备类型检测工具
 * 
 * 提供设备类型识别和管理功能，用于实现单设备类型单点登录。
 */

use serde::{Deserialize, Serialize};
use std::fmt;

/// 设备类型枚举
/// 
/// 用于区分不同类型的客户端设备，实现单设备类型的登录限制。
/// 每种设备类型只能有一个活跃的登录会话。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    /// Web 浏览器
    Web,
    /// 移动应用 (iOS/Android)
    Mobile,
    /// 桌面应用
    Desktop,
    /// API 客户端/其他
    Api,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Web => write!(f, "web"),
            DeviceType::Mobile => write!(f, "mobile"),
            DeviceType::Desktop => write!(f, "desktop"),
            DeviceType::Api => write!(f, "api"),
        }
    }
}

impl DeviceType {
    /// 从字符串解析设备类型
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "web" => DeviceType::Web,
            "mobile" => DeviceType::Mobile,
            "desktop" => DeviceType::Desktop,
            "api" => DeviceType::Api,
            _ => DeviceType::Api, // 默认为 API
        }
    }
}

/// 设备信息结构体
/// 
/// 包含设备的详细信息，用于存储和管理设备会话。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备类型
    pub device_type: DeviceType,
    /// 设备名称或标识
    pub device_name: Option<String>,
    /// User-Agent 字符串
    pub user_agent: Option<String>,
    /// 操作系统信息
    pub os_info: Option<String>,
    /// 浏览器信息（如果是Web设备）
    pub browser_info: Option<String>,
}

impl DeviceInfo {
    /// 从 User-Agent 字符串解析设备信息
    /// 
    /// # 参数
    /// 
    /// * `user_agent` - HTTP 请求中的 User-Agent 字符串
    /// * `device_type_hint` - 可选的设备类型提示（从客户端传入）
    /// 
    /// # 返回值
    /// 
    /// 返回解析后的设备信息
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
    /// let device_info = DeviceInfo::from_user_agent(user_agent, None);
    /// assert_eq!(device_info.device_type, DeviceType::Web);
    /// ```
    pub fn from_user_agent(user_agent: &str, device_type_hint: Option<&str>) -> Self {
        // 如果有明确的设备类型提示，优先使用
        let device_type = if let Some(hint) = device_type_hint {
            DeviceType::from_str(hint)
        } else {
            Self::detect_device_type_from_user_agent(user_agent)
        };

        let (os_info, browser_info) = Self::parse_user_agent_details(user_agent);

        DeviceInfo {
            device_type: device_type.clone(),
            device_name: Self::generate_device_name(&device_type, &os_info, &browser_info),
            user_agent: Some(user_agent.to_string()),
            os_info,
            browser_info,
        }
    }

    /// 从 User-Agent 检测设备类型
    fn detect_device_type_from_user_agent(user_agent: &str) -> DeviceType {
        let ua_lower = user_agent.to_lowercase();

        // 检测移动设备
        if ua_lower.contains("mobile") 
            || ua_lower.contains("iphone") 
            || ua_lower.contains("ipad") 
            || ua_lower.contains("android") 
            || ua_lower.contains("blackberry") 
            || ua_lower.contains("windows phone") {
            return DeviceType::Mobile;
        }

        // 检测桌面应用（Electron 等）
        if ua_lower.contains("electron") 
            || ua_lower.contains("desktop") 
            || ua_lower.contains("app") {
            return DeviceType::Desktop;
        }

        // 检测Web浏览器
        if ua_lower.contains("mozilla") 
            || ua_lower.contains("chrome") 
            || ua_lower.contains("safari") 
            || ua_lower.contains("firefox") 
            || ua_lower.contains("edge") 
            || ua_lower.contains("opera") {
            return DeviceType::Web;
        }

        // 其他情况默认为 API 客户端
        DeviceType::Api
    }

    /// 解析 User-Agent 中的详细信息
    fn parse_user_agent_details(user_agent: &str) -> (Option<String>, Option<String>) {
        let ua_lower = user_agent.to_lowercase();
        
        // 解析操作系统信息
        let os_info = if ua_lower.contains("windows nt 10.0") {
            Some("Windows 10".to_string())
        } else if ua_lower.contains("windows nt 6.3") {
            Some("Windows 8.1".to_string())
        } else if ua_lower.contains("windows nt 6.2") {
            Some("Windows 8".to_string())
        } else if ua_lower.contains("windows nt 6.1") {
            Some("Windows 7".to_string())
        } else if ua_lower.contains("windows") {
            Some("Windows".to_string())
        } else if ua_lower.contains("mac os x") || ua_lower.contains("macos") {
            Some("macOS".to_string())
        } else if ua_lower.contains("linux") {
            Some("Linux".to_string())
        } else if ua_lower.contains("android") {
            Some("Android".to_string())
        } else if ua_lower.contains("ios") || ua_lower.contains("iphone") || ua_lower.contains("ipad") {
            Some("iOS".to_string())
        } else {
            None
        };

        // 解析浏览器信息
        let browser_info = if ua_lower.contains("firefox") {
            Some("Firefox".to_string())
        } else if ua_lower.contains("edg/") {
            Some("Microsoft Edge".to_string())
        } else if ua_lower.contains("chrome") && !ua_lower.contains("edg") {
            Some("Chrome".to_string())
        } else if ua_lower.contains("safari") && !ua_lower.contains("chrome") {
            Some("Safari".to_string())
        } else if ua_lower.contains("opera") {
            Some("Opera".to_string())
        } else {
            None
        };

        (os_info, browser_info)
    }

    /// 生成友好的设备名称
    fn generate_device_name(
        device_type: &DeviceType,
        os_info: &Option<String>,
        browser_info: &Option<String>,
    ) -> Option<String> {
        match device_type {
            DeviceType::Web => {
                match (browser_info, os_info) {
                    (Some(browser), Some(os)) => Some(format!("{} on {}", browser, os)),
                    (Some(browser), None) => Some(browser.clone()),
                    (None, Some(os)) => Some(format!("Browser on {}", os)),
                    (None, None) => Some("Web Browser".to_string()),
                }
            }
            DeviceType::Mobile => {
                match os_info {
                    Some(os) => Some(format!("{} Device", os)),
                    None => Some("Mobile Device".to_string()),
                }
            }
            DeviceType::Desktop => {
                match os_info {
                    Some(os) => Some(format!("Desktop App on {}", os)),
                    None => Some("Desktop App".to_string()),
                }
            }
            DeviceType::Api => Some("API Client".to_string()),
        }
    }

    /// 创建简单的设备信息（用于测试或简单场景）
    pub fn simple(device_type: DeviceType, name: Option<String>) -> Self {
        DeviceInfo {
            device_type,
            device_name: name,
            user_agent: None,
            os_info: None,
            browser_info: None,
        }
    }

    /// 获取设备唯一标识符
    /// 
    /// 用于在 Redis 中区分不同设备类型的会话
    pub fn get_device_key(&self) -> String {
        format!("device:{}", self.device_type)
    }

    /// 获取设备显示名称
    pub fn display_name(&self) -> String {
        self.device_name
            .clone()
            .unwrap_or_else(|| format!("{} Device", self.device_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_detection() {
        // Test web browsers
        let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let device_info = DeviceInfo::from_user_agent(chrome_ua, None);
        assert_eq!(device_info.device_type, DeviceType::Web);
        assert!(device_info.browser_info.as_ref().unwrap().contains("Chrome"));

        // Test mobile devices
        let mobile_ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15";
        let device_info = DeviceInfo::from_user_agent(mobile_ua, None);
        assert_eq!(device_info.device_type, DeviceType::Mobile);

        // Test with hint
        let device_info = DeviceInfo::from_user_agent("Custom Client", Some("desktop"));
        assert_eq!(device_info.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_device_key_generation() {
        let web_device = DeviceInfo::simple(DeviceType::Web, None);
        assert_eq!(web_device.get_device_key(), "device:web");

        let mobile_device = DeviceInfo::simple(DeviceType::Mobile, None);
        assert_eq!(mobile_device.get_device_key(), "device:mobile");
    }
}
