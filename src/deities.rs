use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Deity {
    name: String,
    gender: Gender,
}
