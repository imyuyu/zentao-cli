//! ZenTao CLI 配置管理模块
//!
//! 负责配置的加载、保存和优先级管理
//! 类似 Java Spring 的 @ConfigurationProperties 或 Python 的 configparser
//!
//! # 配置优先级 (高 → 低)
//! 1. 环境变量: ZENTAO_URL, ZENTAO_TOKEN, ZENTAO_PRODUCT_ID, ZENTAO_PROJECT_ID
//! 2. 项目配置: ./.zentao/config.toml
//! 3. 全局配置: ~/.config/zentao-cli/config.toml (Linux/Mac) 或 %APPDATA%/zentao-cli/config.toml (Windows)

use anyhow::Result; // 错误处理，类似 Go 的 error 包装
use serde::{Deserialize, Serialize}; // serde: 序列化/反序列化库，类似 Java 的 Jackson 或 Python 的 pydantic
use std::path::PathBuf; // PathBuf: 可变路径，类似 Java 的 Path 或 Python 的 pathlib.Path

// ============================================================
// 数据结构定义 - 类似 TypeScript 的 interface 或 Java 的 POJO
// ============================================================

/// 配置结构体
///
/// # 类比
/// ```typescript
/// // TypeScript 等价
/// interface Config {
///     url: string;
///     token?: string;       // Optional 等价 Option<String>
///     product_id?: number;   // Optional 等价 Option<u64>
///     project_id?: number;
/// }
/// ```
///
/// ```java
/// // Java 等价
/// public class Config {
///     public String url;
///     public Optional<String> token;
///     public Optional<Long> productId;
///     public Optional<Long> projectId;
/// }
/// ```
///
/// ```python
/// # Python 等价 (dataclass)
/// @dataclass
/// class Config:
///     url: str
///     token: Optional[str] = None
///     product_id: Optional[int] = None
///     project_id: Optional[int] = None
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// ZenTao 服务器地址
    pub url: String,

    /// 认证 token (从 ZenTao API 获取)
    /// #[serde(skip_serializing_if = "Option::is_none")] 表示 None 时不序列化到 TOML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// 默认产品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<u64>,

    /// 默认项目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<u64>,

    /// API 版本 (v1 或 v2)，默认为 v1
    /// v1: Header "Token: xxx"
    /// v2: Header "token: xxx" (小写 t)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// 认证账号（用户名）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl Config {
    /// 获取产品 ID，优先使用传入的值，否则使用配置中的值
    pub fn product_id(&self, cli_value: Option<u64>) -> Option<u64> {
        cli_value.or(self.product_id)
    }

    /// 获取项目 ID，优先使用传入的值，否则使用配置中的值
    pub fn project_id(&self, cli_value: Option<u64>) -> Option<u64> {
        cli_value.or(self.project_id)
    }
}

/// TOML 文件包装结构体
///
/// ZenTao 配置存在 TOML 文件时格式为:
/// ```toml
/// [default]
/// url = "https://xxx.com"
/// token = "xxx"
/// ```
///
/// 因此需要 GlobalConfig 包装一层
///
/// # 类比
/// ```python
/// # Python 等价 - TOML 解析需要包装
/// class GlobalConfig:
///     def __init__(self):
///         self.default: Config = None
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default)]
    pub default: Config,
}

// ============================================================
// 路径函数 - 获取配置文件位置
// ============================================================

/// 获取全局配置文件路径
///
/// # 返回
/// - Linux/Mac: ~/.config/zentao-cli/config.toml
/// - Windows: %APPDATA%/zentao-cli/config.toml
///
/// # 类比
/// ```python
/// # Python 等价
/// from pathlib import Path
///
/// def global_config_path() -> Path:
///     config_dir = Path.home() / ".config" / "zentao-cli"
///     return config_dir / "config.toml"
/// ```
pub fn global_config_path() -> PathBuf {
    // 使用 home_dir() 获取用户主目录，确保跨平台一致性
    // Linux/Mac: ~/.zentao-cli/config.toml
    // Windows: C:\Users\username\.zentao-cli\config.toml
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from(".")) // 兜底当前目录
        .join(".zentao-cli") // join 类似 Path / ".zentao-cli"
        .join("config.toml")
}

/// 获取项目配置文件路径
///
/// # 返回
/// ./.zentao-cli/config.toml (当前目录下的 .zentao-cli 子目录)
pub fn project_config_path() -> PathBuf {
    PathBuf::from(".zentao-cli").join("config.toml")
}

// ============================================================
// 配置合并函数 - 实现配置优先级
// ============================================================

/// 合并配置
///
/// base: 已有配置（优先级较低）
/// override_config: 新配置（优先级较高）
///
/// 只有 override_config 中有值的字段才会覆盖 base
///
/// # 类比
/// ```python
/// # Python 等价
/// def merge_config(base: Config, override_config: Config) -> Config:
///     if override_config.url:  # 非空字符串
///         base.url = override_config.url
///     if override_config.token is not None:
///         base.token = override_config.token
///     if override_config.product_id is not None:
///         base.product_id = override_config.product_id
///     if override_config.project_id is not None:
///         base.project_id = override_config.project_id
///     return base
/// ```
fn merge_config(mut base: Config, override_config: Config) -> Config {
    // 非空字符串表示"已设置"，用于区分"用户设置为空"和"用户没设置"
    if !override_config.url.is_empty() {
        base.url = override_config.url;
    }
    // Option.is_some() 判断是否有值
    if override_config.token.is_some() {
        base.token = override_config.token;
    }
    if override_config.product_id.is_some() {
        base.product_id = override_config.product_id;
    }
    if override_config.project_id.is_some() {
        base.project_id = override_config.project_id;
    }
    if override_config.api_version.is_some() {
        base.api_version = override_config.api_version;
    }
    if override_config.account.is_some() {
        base.account = override_config.account;
    }
    base
}

/// 从 TOML 文件加载配置
///
/// # 参数
/// - path: TOML 文件路径
///
/// # 返回
/// - Ok(Config): 解析成功
/// - Err(anyhow::Error): 解析失败（文件不存在、格式错误等）
///
/// # 类比
/// ```python
/// # Python 等价
/// import tomllib
///
/// def load_toml_config(path: Path) -> Config:
///     with open(path, 'rb') as f:
///         data = tomllib.load(f)
///     return Config(**data['default'])
/// ```
fn load_toml_config(path: &PathBuf) -> Result<Config> {
    // 读取文件内容，类似 Python 的 Path.read_text() 或 Node.js 的 fs.readFileSync
    let content = std::fs::read_to_string(path)?;
    // toml::from_str 解析 TOML，类似 JSON.parse 但针对 TOML 格式
    // 反序列化为 GlobalConfig，再取其中的 default 字段
    let config: GlobalConfig = toml::from_str(&content)?;
    Ok(config.default)
}

// ============================================================
// 主要配置函数
// ============================================================

/// 加载配置（合并所有来源）
///
/// 按优先级从低到高加载：
/// 1. 环境变量（最高优先级）
/// 2. 全局配置 (~/.config/zentao-cli/config.toml)
/// 3. 项目配置 (.zentao/config.toml)
///
/// # 返回
/// 合并后的完整配置
///
/// # 类比
/// ```go
/// // Go 等价
/// func LoadConfig() (*Config, error) {
///     // 1. 环境变量
///     cfg := &Config{
///         URL: os.Getenv("ZENTAO_URL"),
///         Token: os.Getenv("ZENTAO_TOKEN"),
///     }
///     // 2. 全局配置
///     if path := GlobalConfigPath(); exists(path) {
///         if global, err := LoadTomlConfig(path); err == nil {
///             cfg = MergeConfig(cfg, global)
///         }
///     }
///     // 3. 项目配置
///     if path := ProjectConfigPath(); exists(path) {
///         if project, err := LoadTomlConfig(path); err == nil {
///             cfg = MergeConfig(cfg, project)
///         }
///     }
///     return cfg, nil
/// }
/// ```
pub fn load_config() -> Result<Config> {
    // 配置优先级：环境变量 > 项目配置 > 全局配置
    // 先加载全局配置（最低优先级），然后项目配置覆盖，最后环境变量覆盖（最高优先级）

    // 第1步：加载全局配置（最低优先级）
    let mut config = Config {
        url: String::new(),
        token: None,
        product_id: None,
        project_id: None,
        api_version: None,
        account: None,
    };

    let global_path = global_config_path();
    if global_path.exists() {
        if let Ok(global_config) = load_toml_config(&global_path) {
            config = merge_config(config, global_config);
        }
    }

    // 第2步：加载项目配置（中等优先级）
    let project_path = project_config_path();
    if project_path.exists() {
        if let Ok(project_config) = load_toml_config(&project_path) {
            config = merge_config(config, project_config);
        }
    }

    // 第3步：环境变量覆盖（最高优先级）
    if let Ok(url) = std::env::var("ZENTAO_URL") {
        if !url.is_empty() {
            config.url = url;
        }
    }
    if let Ok(token) = std::env::var("ZENTAO_TOKEN") {
        if !token.is_empty() {
            config.token = Some(token);
        }
    }
    if let Ok(product_id) = std::env::var("ZENTAO_PRODUCT_ID") {
        if let Ok(id) = product_id.parse() {
            config.product_id = Some(id);
        }
    }
    if let Ok(project_id) = std::env::var("ZENTAO_PROJECT_ID") {
        if let Ok(id) = project_id.parse() {
            config.project_id = Some(id);
        }
    }
    if let Ok(api_version) = std::env::var("ZENTAO_API_VERSION") {
        if !api_version.is_empty() {
            config.api_version = Some(api_version);
        }
    }

    Ok(config)
}

/// 保存配置到全局配置文件
///
/// # 类比
/// ```python
/// # Python 等价
/// import tomllib
///
/// def save_config(config: Config) -> None:
///     path = global_config_path()
///     path.parent().mkdir(parents=True, exist_ok=True)
///     content = {"default": asdict(config)}
///     with open(path, 'w') as f:
///         tomllib.dump(content, f)
/// ```
/// 保存配置到配置文件
/// - global=true: 保存到全局配置
/// - global=false: 保存到项目配置（自动创建目录如果不存在）
fn save_config_to_file(config: &Config, global: bool) -> Result<PathBuf> {
    let path = if global {
        global_config_path()
    } else {
        project_config_path()
    };

    // 确保目录存在，create_dir_all 类似 Go 的 os.MkdirAll 或 Python 的 mkdir -p
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 包装成 GlobalConfig 格式
    let global = GlobalConfig {
        default: config.clone(), // clone 类似 Python 的 copy.deepcopy
    };

    // 序列化为 TOML 字符串，类似 JSON.stringify 但输出 TOML 格式
    let content = toml::to_string_pretty(&global)?;
    // 写入文件，类似 Python 的 Path.write_text()
    std::fs::write(&path, content)?;

    Ok(path)
}

pub fn save_config(config: &Config) -> Result<()> {
    save_config_to_file(config, false)?;
    Ok(())
}

/// 更新单个配置项并保存
///
/// # 参数
/// - key: 配置键名 (url/token/product_id/project_id)
/// - value: 配置值
/// - global: true 保存到全局配置，false 保存到项目配置（如果存在）
///
/// # 返回
/// 更新单个配置项并保存
///
/// # 参数
/// - key: 配置键名 (url/token/product_id/project_id)
/// - value: 配置值
/// - global: true 保存到全局配置，false 保存到项目配置（如果存在）
///
/// # 返回
/// 保存配置的路径
pub fn update_config(key: &str, value: &str, global: bool) -> Result<PathBuf> {
    let mut config = load_config()?;

    match key {
        "url" => {
            config.url = value.to_string();
        }
        "token" => {
            config.token = Some(value.to_string());
        }
        "product_id" => {
            // parse().ok() 类似 Python 的 try int(value) catch 返回 None
            config.product_id = value.parse().ok();
        }
        "project_id" => {
            config.project_id = value.parse().ok();
        }
        "api_version" => {
            config.api_version = Some(value.to_string());
        }
        "account" => {
            config.account = Some(value.to_string());
        }
        _ => {
            anyhow::bail!("Unknown config key: {}", key); // 类似 Go 的 errors.New()
        }
    }

    save_config_to_file(&config, global)
}

/// 删除配置项并保存
///
/// # 类比
/// ```python
/// # Python 等价
/// def unset_config(key: str) -> Config:
///     config = load_config()
///     if key == "url":
///         config.url = ""
///     elif key == "token":
///         config.token = None
///     elif key in ("product_id", "project_id"):
///         setattr(config, key, None)
///     save_config(config)
///     return config
/// ```
pub fn unset_config(key: &str, global: bool) -> Result<PathBuf> {
    let mut config = load_config()?;

    match key {
        "url" => {
            config.url = String::new(); // 空字符串
        }
        "token" => {
            config.token = None; // None 类似 null 或 undefined
        }
        "product_id" => {
            config.product_id = None;
        }
        "project_id" => {
            config.project_id = None;
        }
        "api_version" => {
            config.api_version = None;
        }
        "account" => {
            config.account = None;
        }
        _ => {
            anyhow::bail!("Unknown config key: {}", key);
        }
    }

    save_config_to_file(&config, global)
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试配置合并逻辑
    #[test]
    fn test_merge_config() {
        let base = Config {
            url: "http://base".to_string(),
            token: Some("base_token".to_string()),
            product_id: Some(1),
            project_id: None,
            api_version: None,
            account: Some("base_user".to_string()),
        };

        let override_config = Config {
            url: "http://override".to_string(),
            token: None, // None 表示不覆盖
            product_id: Some(2),
            project_id: Some(10),
            api_version: None,
            account: Some("override_user".to_string()),
        };

        let result = merge_config(base, override_config);

        // 验证 override 生效
        assert_eq!(result.url, "http://override");
        // token 为 None 所以不覆盖，保留 base 值
        assert_eq!(result.token, Some("base_token".to_string()));
        assert_eq!(result.product_id, Some(2));
        assert_eq!(result.project_id, Some(10));
        assert_eq!(result.account, Some("override_user".to_string()));
    }

    /// 测试默认配置
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.url.is_empty());
        assert!(config.token.is_none());
        assert!(config.product_id.is_none());
        assert!(config.project_id.is_none());
        assert!(config.account.is_none());
    }
}
