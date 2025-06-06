use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentRating {
    Everyone,
    Mature,
    Adult,
    Multi,
}
