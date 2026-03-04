use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<PaginationMeta>,
    timestamp: String,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub total_pages: usize,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            meta: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_pagination(data: T, meta: PaginationMeta) -> Self {
        Self {
            data,
            meta: Some(meta),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
