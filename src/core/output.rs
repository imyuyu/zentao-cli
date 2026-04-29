use clap::ValueEnum;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// 完整 JSON 响应
    #[default]
    Json,
    /// 人性化格式输出
    Pretty,
    /// 易读表格
    Table,
    /// 换行分隔 JSON（适合管道处理）
    Ndjson,
    /// 逗号分隔值
    Csv,
}

impl OutputFormat {
    /// 判断是否为 JSON 系列格式（可用于流式输出）
    pub fn is_json_like(&self) -> bool {
        matches!(self, OutputFormat::Json | OutputFormat::Ndjson)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub hint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".into(),
            data: Some(data),
            meta: None,
            error: None,
        }
    }

    pub fn with_meta(data: T, meta: PaginationMeta) -> Self {
        Self {
            status: "success".into(),
            data: Some(data),
            meta: Some(meta),
            error: None,
        }
    }

    pub fn error(code: &str, message: &str, hint: &str) -> Self {
        Self {
            status: "error".into(),
            data: None,
            meta: None,
            error: Some(ErrorDetail {
                code: code.into(),
                message: message.into(),
                hint: hint.into(),
            }),
        }
    }

}

pub fn print_json<T: Serialize + DeserializeOwned>(value: &T) -> anyhow::Result<String> {
    let json = serde_json::to_string_pretty(value)?;
    Ok(json)
}

pub fn print_table<T: Serialize>(items: &[T], columns: &[&str]) -> String {
    if items.is_empty() {
        return String::new();
    }

    let mut table = String::new();
    let json_items: Vec<serde_json::Value> = items
        .iter()
        .map(|item| serde_json::to_value(item).unwrap_or_default())
        .collect();

    // Print header
    table.push_str(&columns.join("\t"));
    table.push('\n');

    // Print rows
    for item in &json_items {
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                item.get(*col)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => String::new(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        table.push_str(&row.join("\t"));
        table.push('\n');
    }

    table
}

/// 打印 NDJSON 格式（每行一个 JSON 对象）
pub fn print_ndjson<T: Serialize>(items: &[T]) -> String {
    items
        .iter()
        .map(|item| serde_json::to_string(item).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}

/// 打印 CSV 格式
pub fn print_csv<T: Serialize>(items: &[T], columns: &[&str]) -> String {
    if items.is_empty() {
        return String::new();
    }

    let mut csv = String::new();

    // Print header
    csv.push_str(&columns.join(","));
    csv.push('\n');

    // Print rows
    for item in items {
        let json_value = serde_json::to_value(item).unwrap_or_default();
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                json_value
                    .get(*col)
                    .map(|v| match v {
                        serde_json::Value::String(s) => {
                            // CSV 中需要转义引号和换行
                            format!("\"{}\"", s.replace('"', "\"\""))
                        }
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        serde_json::Value::Null => String::new(),
                        _ => v.to_string(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        csv.push_str(&row.join(","));
        csv.push('\n');
    }

    csv
}

/// 根据格式输出
pub fn format_output<T: Serialize>(items: &[T], format: OutputFormat, columns: &[&str]) -> String {
    match format {
        OutputFormat::Json | OutputFormat::Pretty => serde_json::to_string_pretty(items).unwrap_or_default(),
        OutputFormat::Ndjson => print_ndjson(items),
        OutputFormat::Csv => print_csv(items, columns),
        OutputFormat::Table => print_table(items, columns),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestItem {
        id: u64,
        name: String,
    }

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success(TestItem {
            id: 1,
            name: "test".to_string(),
        });
        assert_eq!(response.status, "success");
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_with_meta() {
        let response = ApiResponse::with_meta(
            TestItem {
                id: 1,
                name: "test".to_string(),
            },
            PaginationMeta {
                total: 100,
                page: 1,
                page_size: 10,
            },
        );
        assert_eq!(response.status, "success");
        assert!(response.meta.is_some());
        let meta = response.meta.unwrap();
        assert_eq!(meta.total, 100);
        assert_eq!(meta.page, 1);
    }

    #[test]
    fn test_api_response_error() {
        let response = ApiResponse::<()>::error("ERR_CODE", "error message", "hint message");
        assert_eq!(response.status, "error");
        assert!(response.data.is_none());
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, "ERR_CODE");
        assert_eq!(error.message, "error message");
    }

    #[test]
    fn test_output_format_default() {
        let format = OutputFormat::default();
        assert_eq!(format, OutputFormat::Json);
    }

    #[test]
    fn test_pagination_meta_serde() {
        let meta = PaginationMeta {
            total: 50,
            page: 2,
            page_size: 20,
        };
        let json = serde_json::to_string(&meta).unwrap();
        let parsed: PaginationMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.total, 50);
        assert_eq!(parsed.page, 2);
    }

    #[test]
    fn test_error_detail_serde() {
        let detail = ErrorDetail {
            code: "TEST_ERR".to_string(),
            message: "Test message".to_string(),
            hint: "Test hint".to_string(),
        };
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("TEST_ERR"));
        assert!(json.contains("Test message"));
    }
}
