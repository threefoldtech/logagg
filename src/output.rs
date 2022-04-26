use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Output {
    #[serde(alias = "type")]
    pub typ: String,
    pub dir: String,
}