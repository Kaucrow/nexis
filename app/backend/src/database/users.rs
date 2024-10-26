use anyhow::{ anyhow, Result };
use mongodb::{ self, bson::oid::ObjectId, Collection };
use crate::types::{ User, NewUser };

#[tracing::instrument(
    name = "Inserting new user into DB.",
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

    let res = user_coll.insert_one(User::try_from(new_user)?).await?;

    tracing::info!(target: "mongodb", "User profile created successfully {}.", res.inserted_id);

    if let Some(oid) = res.inserted_id.as_object_id() { 
        Ok(oid)
    } else {
        Err(anyhow!("Failed to retrieve ObjectId"))
    }
}