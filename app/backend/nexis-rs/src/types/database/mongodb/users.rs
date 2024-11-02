use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Utc };
use mongodb::bson::oid::ObjectId;
use crate::types::requests::users::NewUser;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    #[serde(rename = "dateAdded")]
    date_added: DateTime<Utc>,
    coll: String,
    item: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    stars: u8,
    title: String,
    comment: String,
    coll: String,
    item: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub interests: Vec<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Box<Vec<CartItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Box<Vec<Review>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schedule {
    /*#[serde(rename = "enter")]
    pub enter_date: DateTime<Utc>,
    #[serde(rename = "exit")]
    pub exit_date: DateTime<Utc>,*/
    pub store: ObjectId,
    pub job: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub age: i32,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub username: String,
    pub password: String,
    pub name: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<Box<Employee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Box<Admin>>,
}

impl User {
    pub fn get_roles(&self) -> Vec<&'static str> {
        let mut roles: Vec<&'static str> = Vec::new();

        if self.client.is_some() { roles.push("client"); }
        if self.employee.is_some() { roles.push("employee"); }
        if self.admin.is_some() { roles.push("admin"); }

        roles
    }
}

impl TryFrom<NewUser> for User {
    type Error = anyhow::Error;

    fn try_from(new: NewUser) -> std::result::Result<Self, Self::Error> {
        let client =
            if let Some(new) = new.client {
                Some(Box::new(Client {
                    age: new.age,
                    gender: new.gender,
                    phone_num: new.phone_num,
                    interests: new.interests,
                    cart: None,
                    reviews: None,
                }))
            } else {
                None
            };

        let employee =
            if let Some(new) = new.employee {
                Some(Box::new(Employee {
                    age: new.age,
                    gender: new.gender,
                    phone_num: new.phone_num,
                    schedule: new.schedule,
                }))
            } else {
                None
            };

        let admin =
            if let Some(_new) = new.admin {
                Some(Box::new(Admin {}))
            } else {
                None
            };
        
        if client.is_some() || employee.is_some() || admin.is_some() {
            Ok(
                User {
                    id: ObjectId::new(),
                    email: new.email,
                    username: new.username,
                    password: new.password,
                    name: new.name,
                    is_active: false,
                    client,
                    employee,
                    admin,
                }
            )
        } else {
            Err(anyhow!("New user has no role."))
        }
    }
}