use crate::prelude::*;
use crate::types::{ Role, mongodb::users::Schedule };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewClient {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub interests: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEmployee {
    pub age: i32,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAdmin {
    pub stores: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<NewClient>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<Box<NewEmployee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Box<NewAdmin>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser {
    pub identifier: String,
    pub password: String,
    #[serde(rename = "rememberMe")]
    pub remember_me: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoleLoginUser {
    #[serde(rename = "token")]
    pub rolesel_pub_token: String,
    pub role: Role,
}