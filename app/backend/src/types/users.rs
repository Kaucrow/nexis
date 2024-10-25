use anyhow::{ anyhow, Result };
use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Utc };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
struct CartItem {
    #[serde(rename = "dateAdded")]
    date_added: DateTime<Utc>,
    coll: String,
    item: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
struct Review {
    stars: u8,
    title: String,
    comment: String,
    coll: String,
    item: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
struct Client {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub interests: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Box<Vec<CartItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Box<Vec<Review>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Schedule {
    #[serde(rename = "phoneNum")]
    pub enter_date: DateTime<Utc>,
    pub exit_date: DateTime<Utc>,
    pub store: ObjectId,
    #[serde(rename = "storeJob")]
    pub store_job: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Admin {}

#[derive(Debug, Serialize, Deserialize)]
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
    client: Option<Box<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    employee: Option<Box<Employee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin: Option<Box<Admin>>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct UserVisible {
    pub id: ObjectId,
    pub email: String,
    pub name: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewClient {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub interests: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEmployee {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    schedule: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAdmin {}

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
    pub email: String,
    pub password: String,
}