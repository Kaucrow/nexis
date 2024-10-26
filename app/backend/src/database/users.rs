use crate::prelude::*;
use anyhow::Result;
use types::{ User, NewUser };

#[tracing::instrument(
    name = "Inserting new user into DB",
    skip(db, new_user),
    fields(
        new_user_email = %new_user.email,
        new_user_name = %new_user.name
    )
)]
pub async fn insert_created_user_into_db(
    db: &mongodb::Database,
    new_user: NewUser
) -> Result<ObjectId> {
    let user_coll: Collection<User> = db.collection("user");

    // Check if a user with the same email or username already exists
    let existing_user = user_coll
        .find_one(
            doc! {
                "$or": [
                    { "email": &new_user.email },
                    { "username": &new_user.username },
                ]
            },
        )
        .await?;

    if existing_user.is_some() {
        bail!(types::error::Mongodb::UserAlreadyExists(
            "A user with this username or email already exists.".into()
        ))
    }

    let res = user_coll.insert_one(User::try_from(new_user)?).await?;

    tracing::info!(target: "mongodb", "User profile created successfully {}.", res.inserted_id);

    if let Some(oid) = res.inserted_id.as_object_id() { 
        Ok(oid)
    } else {
        Err(anyhow!("Failed to retrieve ObjectId"))
    }
}

#[tracing::instrument(name = "Getting user from DB")]
pub async fn get_db_user(
    db: &mongodb::Database,
    user_id: ObjectId,
) -> Result<Option<User>> {
    let users_coll: Collection<User> = db.collection("user");

    let user = users_coll.find_one(
        doc! { "_id": user_id }
    ).await?;

    Ok(user)
}