use crate::prelude::*;
use anyhow::Result;
use types::{
    requests,
    database::mongodb::{ User, IsCollection },
};

#[derive(Debug)]
pub enum NewUser {
    Client(requests::NewClient),
    Employee(requests::NewEmployee),
}

#[tracing::instrument(
    name = "Inserting new user into DB",
    skip(db, new_user),
)]
pub async fn insert_created_user_into_db(
    db: &mongodb::Database,
    new_user: NewUser,
) -> Result<ObjectId> {
    let user_coll: Collection<User> = db.collection(User::coll_name());

    let user = User::try_from(new_user)?;

    tracing::debug!(target: "mongodb", "Inserting user into DB: {{ email: {}, username: {} }}", user.email, user.username);

    // Check if a user with the same email or username already exists
    let existing_user = user_coll
        .find_one(
            doc! {
                "$or": [
                    { "email": &user.email },
                    { "username": &user.username },
                ]
            },
        )
        .await?;

    if existing_user.is_some() {
        bail!(types::error::Mongodb::UserAlreadyExists(
            "A user with this username or email already exists.".into()
        ))
    }

    let res = user_coll.insert_one(user).await?;

    tracing::info!(target: "mongodb", "User profile created successfully {}.", res.inserted_id);

    if let Some(oid) = res.inserted_id.as_object_id() { 
        Ok(oid)
    } else {
        Err(anyhow!("Failed to retrieve ObjectId"))
    }
}

#[tracing::instrument(name = "Getting user from DB")]
pub async fn get_user(
    db: &mongodb::Database,
    user_id: ObjectId,
) -> Result<Option<User>> {
    let users_coll: Collection<User> = db.collection(User::coll_name());

    let user = users_coll.find_one(
        doc! { "_id": user_id }
    ).await?;

    Ok(user)
}

#[tracing::instrument(name = "Getting client cart from DB")]
pub async fn get_client_cart(
    db: &mongodb::Database,
    user_id: ObjectId,
) -> Result<Document> {
    let users_coll: Collection<Document> = db.collection(User::coll_name());

    let mut cursor = users_coll.aggregate(vec![
        doc! { "$match": { "_id": user_id }},
        doc! { "$project": { "_id": 0, "client": { "cart": 1 }}},
    ]).await?;

    if let Ok(Some(doc)) = cursor.try_next().await {
        Ok(doc)
    } else {
        bail!("The user has no client data.")
    }
}