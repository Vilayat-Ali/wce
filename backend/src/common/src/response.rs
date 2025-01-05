use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct SuccessResponse {
    pub status: u16,
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SuccessDataResponse<T>
where
    T: Serialize,
{
    pub status: u16,
    pub message: String,
    pub data: T,
}
