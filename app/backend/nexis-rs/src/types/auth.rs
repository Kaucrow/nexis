use std::{ str::FromStr, fmt };

use crate::prelude::*;
use types::User;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Client,
    Admin,
    Employee
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Client => write!(f, "client"),
            Role::Employee => write!(f, "employee"),
            Role::Admin => write!(f, "admin"),
        } 
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        match self {
            Role::Client => serializer.serialize_unit_variant("Role", 0, "client"),
            Role::Employee => serializer.serialize_unit_variant("Role", 1, "employee"),
            Role::Admin => serializer.serialize_unit_variant("Role", 2, "admin"),
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: serde::Deserializer<'de>
    {
        struct RoleVisitor;

        impl<'de> serde::de::Visitor<'de> for RoleVisitor {
            type Value = Role;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`client`, `employee`, or `admin`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Role, E>
            where E: serde::de::Error,
            {
                match value {
                    "client" => Ok(Role::Client),
                    "employee" => Ok(Role::Employee),
                    "admin" => Ok(Role::Admin),
                    _ => Err(serde::de::Error::unknown_field(
                        value, &["client", "employee", "admin"]
                    ))
                }
            }
        }

        deserializer.deserialize_identifier(RoleVisitor)
    }
}

impl FromStr for Role {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "client" => Ok(Role::Client),
            "employee" => Ok(Role::Employee),
            "admin" => Ok(Role::Admin),
            _ => bail!("Unknown role.")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Session {
    Client(ClientSession),
    Admin(AdminSession),
    Employee(EmployeeSession),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientSession {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeSession {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminSession {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub id: ObjectId,
    pub role: Role,
    pub data: Session,
}

impl UserSession {
    pub fn try_from(user: User, role: Role) -> Result<Self> {
        let data = match role {
            Role::Client => {
                user.client
                    .map(|_| Session::Client(ClientSession {}))
                    .ok_or_else(|| anyhow!("User lacks client role data."))?
            }
            Role::Employee => {
                user.employee
                    .map(|_| Session::Employee(EmployeeSession {}))
                    .ok_or_else(|| anyhow!("User lacks employee role data."))?
            }
            Role::Admin => {
                user.admin
                    .map(|_| Session::Admin(AdminSession {}))
                    .ok_or_else(|| anyhow!("User lacks admin role data."))?
            }
        };

        Ok(UserSession {
            id: user.id,
            role,
            data,
        })
    }
}