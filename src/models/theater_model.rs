use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Theater {
    pub id: String,
    pub name: String,
    pub location: String,
}
