use serde::Serialize;

/// 统一响应实体 - 所有接口都使用此实体返回
/// T 为泛型，可以是任意类型：数字、字符串、null、集合、对象等
#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// 创建成功响应
impl<T> Response<T> {
    /// 成功响应（带数据）
    pub fn success(data: T) -> Self {
        Response {
            success: true,
            message: "操作成功".to_string(),
            data: Some(data),
        }
    }

    /// 成功响应（带数据和自定义消息）
    pub fn success_with_message(data: T, message: String) -> Self {
        Response {
            success: true,
            message,
            data: Some(data),
        }
    }

    /// 成功响应（无数据）
    pub fn success_empty() -> Response<()> {
        Response::<()> {
            success: true,
            message: "操作成功".to_string(),
            data: None,
        }
    }

    /// 成功响应（无数据，自定义消息）
    pub fn success_empty_with_message(message: String) -> Response<()> {
        Response::<()> {
            success: true,
            message,
            data: None,
        }
    }

    /// 错误响应
    pub fn error(message: String) -> Self {
        Response {
            success: false,
            message,
            data: None,
        }
    }
}

