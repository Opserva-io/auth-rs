use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SearchRequest {
    pub text: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}
