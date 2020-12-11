extern crate serde_derive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "Saves")]
    pub saves: Vec<Save>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: String,
}
