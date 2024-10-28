pub mod common;
pub mod database;
pub mod responses;
pub mod requests;
pub mod error;

pub use common::{ USER_ID_KEY, USER_EMAIL_KEY };
pub use database::mongodb::users::User;
pub use responses::{ SuccessResponse, ErrorResponse, UserResponse };
pub use requests::users::{ NewUser, LoginUser };

use database::mongodb;