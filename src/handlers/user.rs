use actix_multipart::Multipart;
use tokio::io::AsyncWriteExt;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use crate::state::State;
use actix_web::{Scope,
    web::{self, delete, get, post, put, resource, scope},
    HttpResponse, HttpRequest
};
use div_db::models::User;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserApi(User);


pub fn uid_routes() -> Scope {
    scope("/user")
    // -------------- /user ------------------------- ///
        .service(resource("").route(get().to(get_all)))
        .service(scope("/{uid}")
        // -------------- /user/{uid} --------------------///
            .service(resource("")
                .route(get().to(get_by_id))
                .route(delete().to(delete_by_id))
            )
            // ------------ /user/{uid}/info/ -------- ///
            .service(resource("/info")
                .route(get().to(get_user_info))
                .route(put().to(update_user_info))
            )
            .service(scope("/facts")
                .route(get().to(get_uid_facts))
                .route(put().to(new_uid_fact))
            )
        )
}

pub async fn test() {}

pub fn username_routes() -> Scope {
    // -------------- /u ------------------------- ///
    scope("/u")
        .service(resource("").route(get().to(|| HttpResponse::Ok().body("/u"))))
        // -------------- /u/{username} --------------------///
        .service(scope("/{username}")
            .service(resource("")
                .route(get().to(get_by_username))
                .route(put().to(update_by_username))
                .route(delete().to(delete_by_username)),
            )
            // ------------ /u/{username}/feed/ -------- ///
            .service(scope("/feed")
                .service(resource("")
                    .route(get().to(get_user_feed))
                )
                // ------------ /u/{username}/feed/items -------- ///
                .service(resource("/items")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
                // ------------ /u/{username}/feed/entities -------- ///
                .service(resource("/entities")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
                // ------------ /u/{username}/feed/records -------- ///
                .service(resource("/records")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
            )
        )
}

//TODO programmatically handle requests by matching operation to user model function

pub async fn get_all(
    id: actix_session::Session,
    data: web::Data<State>,) -> HttpResponse
{
    //println!("GET ALL: FROM {:?}", id.identity());
    let db = &data.db.lock().unwrap();
    match User::get_all(&db).await {
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&users),
        Err(_) => HttpResponse::NotFound().json("")
    }
}

pub async fn get_by_id(data: web::Data<State>, id: web::Path<String>) -> HttpResponse {
    let id: Uuid = Uuid::parse_str(id.into_inner().as_mut_str()).unwrap();
    match User::get_by_id(&data.db.lock().unwrap(), id).await {
        Ok(maybe_user) => match maybe_user {
            Some(user) => HttpResponse::Ok()
                .content_type("application/json")
                .json(&user),
            None => HttpResponse::NotFound().json(""),
        },
        Err(_) => HttpResponse::NotFound().json(""),
    }
}

pub async fn update_by_id(
    path: web::Path<Uuid>, req: HttpRequest, data: web::Data<State>
        ) -> HttpResponse
{
    match User::delete_by_id(&data.db.lock().unwrap(), *path).await {
        Ok(Some(id)) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user with id {}", id)),
        _ => HttpResponse::NotFound().body("Could not delete")
    }
}

pub async fn delete_by_id(
    data: web::Data<State>,
    id: web::Path<Uuid>
) -> HttpResponse {
    match User::delete_by_id(&data.db.lock().unwrap(), *id).await {
        Ok(Some(id)) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {:?}", id)),
        _ => HttpResponse::NotFound().json("")
    }
}

pub async fn get_by_username(
    data: web::Data<State>,
    username: web::Path<String>
) -> HttpResponse {
    match User::get_by_username(&data.db.lock().unwrap(), username.to_string()).await {
        Ok(Some(user)) => HttpResponse::Ok()
                .content_type("application/json")
                .json(&user),
        _ => HttpResponse::NotFound().json(""),
    }
}

pub async fn delete_by_username(
    id: actix_session::Session,
    data: web::Data<State>,
    username: web::Path<String>,
) -> HttpResponse {
    println!("DELETE USER BY USERNAME: From {:?}", id.get::<usize>("id"));
    match User::delete_by_username(&data.db.lock().unwrap(), username.to_string()).await {
        Ok(id) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id)),
        Err(_) => HttpResponse::NotFound().json("")
    }
}


pub async fn update_by_username(
    id: actix_session::Session,
    data: web::Data<State>,
    username: web::Path<String>
) -> HttpResponse {
    println!("UPDATE USER BY USERNAME: From {:?}", id.get::<usize>("id"));
    match User::delete_by_username(&data.db.lock().unwrap(), username.to_string()).await {
        Ok(id) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id)),
        Err(_) => HttpResponse::NotFound().json("")
    }
}


pub async fn get_user_info(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn update_user_info(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn get_user_feed(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn get_uid_facts(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn new_uid_fact(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

async fn upload_profile_picture(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", &filename);
        let mut f = tokio::fs::File::create(&filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub async fn get_user_feed(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}
