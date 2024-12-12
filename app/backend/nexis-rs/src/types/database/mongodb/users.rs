use crate::prelude::*;
use super::IsCollection;
use types::auth::Role;
use utils::database::users::NewUser;
use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Utc };
use mongodb::bson::oid::ObjectId;

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
    pub interests: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Box<Vec<CartItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Box<Vec<Review>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schedule {
    #[serde(rename = "enter")]
    pub enter_date: DateTime<Utc>,
    #[serde(rename = "exit")]
    pub exit_date: DateTime<Utc>,
    #[serde(rename = "checkedIn", skip_serializing_if = "Option::is_none")]
    pub checked_in: Option<DateTime<Utc>>,
    #[serde(rename = "checkedOut", skip_serializing_if = "Option::is_none")]
    pub checked_out: Option<DateTime<Utc>>,
    pub store: String,
    pub job: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(rename = "payPerWeek")]
    pub pay: f64,
    pub stores: Vec<String>,
}

impl IsCollection for Job {
    fn coll_name() -> &'static str { "storeJobs" }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub age: u8,
    pub gender: String,
    #[serde(rename = "phoneNum")]
    pub phone_num: String,
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub stores: Vec<String>,
}

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
    pub fn get_roles(&self) -> Vec<Role> {
        let mut roles: Vec<Role> = Vec::new();

        if self.client.is_some() { roles.push(Role::Client); }
        if self.employee.is_some() { roles.push(Role::Employee); }
        if self.admin.is_some() { roles.push(Role::Admin); }

        roles
    }
}

impl IsCollection for User {
    fn coll_name() -> &'static str { "users" }
}

impl TryFrom<NewUser> for User {
    type Error = anyhow::Error;

    fn try_from(new: NewUser) -> std::result::Result<Self, Self::Error> {
        match new {
            NewUser::Client(new) => Ok(User {
                id: ObjectId::new(),
                email: new.email,
                password: new.password,
                username: new.username,
                name: new.name,
                is_active: false,
                client: Some(Box::new(Client {
                    age: new.age,
                    gender: new.gender,
                    phone_num: new.phone_num,
                    interests: new.interests,
                    cart: None,
                    reviews: None,
                })),
                employee: None,
                admin: None,
            }),
            NewUser::Employee(new) => Ok(User {
                id: ObjectId::new(),
                email: new.email,
                password: new.password,
                username: new.username,
                name: new.name,
                is_active: false,
                client: None,
                employee: Some(Box::new(Employee {
                    age: new.age,
                    gender: new.gender,
                    phone_num: new.phone_num,
                    schedule: new.schedule,
                })),
                admin: None,
            })
        }
    }
}