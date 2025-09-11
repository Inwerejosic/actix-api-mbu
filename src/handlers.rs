use crate::database::establish_connection;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use crate::model::{Member, NewMember, UpdateMember};

// Get all Handler
pub async fn get_member() -> Result<HttpResponse> {
    use crate::schema::member::dsl::*;

    let mut connection = establish_connection();
    let members = member
        .load::<Member>(&mut connection)
        .expect("Error Loading members");
    Ok(HttpResponse::Ok().json(members))
}

// Get one Handler
pub async fn get_member_by_id(path_id: web::Path<i32>) -> Result<HttpResponse> {
    use crate::schema::member::dsl::*;

    let mut connection = establish_connection();

    match member.find(*path_id).first::<Member>(&mut connection) {
        Ok(h) => Ok(HttpResponse::Ok().json(h)),
        Err(_) => Ok(HttpResponse::NotFound().body(format!("Member with id {} not found", path_id))),
    }
}

// Create Handler
pub async fn create_member(new_member: web::Json<NewMember>) -> Result<HttpResponse> {
    use crate::schema::member::dsl::*;

    let mut connection = establish_connection();

    diesel::insert_into(member)
        .values(new_member.into_inner())
        .execute(&mut connection)
        .expect("Error inserting new member");

    Ok(HttpResponse::Ok().json("Data inserted into the Team database"))
}

// Update Handler
pub async fn update_member(
    path_id: web::Path<i32>,
    member_update: web::Json<UpdateMember>,
) -> Result<HttpResponse> {
    use crate::schema::member::dsl::*;

    let mut connection = establish_connection();

    diesel::update(member.find(*path_id))
        .set(&member_update.into_inner())
        .execute(&mut connection)
        .expect("Failed to update member");

    // Return the updated record instead of just the number of rows updated
    let updated = member
        .find(*path_id)
        .first::<Member>(&mut connection)
        .expect("Failed to fetch updated member");

    Ok(HttpResponse::Ok().json(updated))
}

// Delete Handler
pub async fn delete_member(path_id: web::Path<i32>) -> Result<HttpResponse> {
    use crate::schema::member::dsl::*;
    let mut connection = establish_connection();

    diesel::delete(member.find(*path_id))
        .execute(&mut connection)
        .expect(&format!("Unable to find member {:?}", path_id));

    Ok(HttpResponse::Ok().json("Deleted Successfully"))
}
