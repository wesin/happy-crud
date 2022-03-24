use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PersonAddDO {
    pub name: String,
    pub age: Option<i32>,
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PersonQueryDO {
    pub name: Option<String>,
}
