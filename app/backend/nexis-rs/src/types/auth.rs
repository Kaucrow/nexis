use crate::prelude::*;
use types::{ User, mongodb::CartItem };
use anyhow::Result;

pub enum Roles {
    Client,
    Admin,
    Employee
}

pub enum Session {
    Client(ClientSession),
    Admin(AdminSession),
    Employee(EmployeeSession),
}

pub struct ClientSession {}

pub struct EmployeeSession {}

pub struct AdminSession {}

pub struct UserSession {
    id: ObjectId,
    role: String,
    data: Session,
}

impl UserSession {
    pub fn try_from(user: User, role: String) -> Result<Self> {
        let data = match role.as_str() {
            "client" => {
                user.client
                    .map(|_| Session::Client(ClientSession {}))
                    .ok_or_else(|| anyhow!("User lacks client role data."))?
            }
            "employee" => {
                user.employee
                    .map(|_| Session::Employee(EmployeeSession {}))
                    .ok_or_else(|| anyhow!("User lacks employee role data."))?
            }
            "admin" => {
                user.employee
                    .map(|_| Session::Employee(EmployeeSession {}))
                    .ok_or_else(|| anyhow!("User lacks employee role data."))?
            }
            _ => bail!("Unknown role")
        };

        Ok(UserSession {
            id: user.id,
            role,
            data,
        })
    }
}