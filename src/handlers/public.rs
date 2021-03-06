pub mod admin;
pub mod dashboard;
pub mod auth;

use uuid::Uuid;
use std::collections::HashMap;
use crate::state::State;
use actix_web::{ get,
    web::{self, ServiceConfig},
    HttpResponse,
};
use div_db::models::User;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(index)
        .service(users)
        .service(user)
        .service(cover)
        .service(contact)
        .service(dash)
        .service(about)
        .service(self::auth::routes(""))
        .service(self::dashboard::routes("/dashboard"))
        .service(self::admin::routes("/admin"));
}

#[get("/")]
pub async fn index(
    id: actix_session::Session,
    req: actix_web::HttpRequest,
    data: web::Data<State>,) -> HttpResponse
{
    let db = data.db.lock().unwrap();
    let h = req.headers().into_iter()
        .fold(HashMap::new(), |mut hm, (h, v)| {
            hm.insert(h.to_string(), v.to_str().unwrap_or_default().to_string());
            hm
        });
    let mut ctx = tera::Context::new();
    ctx.insert("host", req.connection_info().host());
    ctx.insert("remote", &req.connection_info().remote_addr());
    ctx.insert("peer", &req.peer_addr().unwrap());
    ctx.insert("scheme", req.connection_info().scheme());
    let uid = crate::session::id(&id).unwrap_or(Uuid::nil());
    ctx.insert("uid", &uid.to_string());
    let s = data.tera.render("index.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
        .unwrap_or_default();
    HttpResponse::Ok().content_type("text/html").body(s)
}


#[get("/about")]
pub async fn about(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("contact.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/contact")]
pub async fn contact(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("contact.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/cover")]
pub async fn cover(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("cover.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/users")]
pub async fn users(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let db = data.db.lock().unwrap();
    let users = User::get_all(&db).await.unwrap_or_default();
    let mut ctx = tera::Context::new();
    ctx.insert("users", &users.to_owned());
    let s = data.tera.render("users.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/users/{username}")]
pub async fn user(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    username: web::Path<String>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let db = data.db.lock().unwrap();
    let user = User::get_by_username(&db, username.into_inner()).await.unwrap_or_default();
    let mut ctx = tera::Context::new();
    if let Some(user) = user {
        ctx.insert("user", &user.to_owned());
        let s = data.tera.render("user.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        ctx.insert("status_code", &"404".to_string());
        ctx.insert("error", &"No user found".to_string());
        let s = data.tera.render("error.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}

#[get("/dashboard")]
pub async fn dash(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> HttpResponse
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("dashboard.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
            .unwrap_or_default();
    HttpResponse::Ok().content_type("text/html").body(s)
}
