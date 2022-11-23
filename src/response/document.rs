pub mod documents;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RequestTitle {
    pub id: String,
    pub code: String,
    pub name: String,
}
