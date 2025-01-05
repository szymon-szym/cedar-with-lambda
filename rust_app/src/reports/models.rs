use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) owner_id: String,
    pub(crate) s3_key: String,
}