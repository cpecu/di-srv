use serde::{Serialize, Deserialize};
use crate::state::State;
use actix_web::{
    HttpRequest, HttpResponse, web
};
use div_db::models::{Model, user::*};

pub struct AuthRequest<T: Serialize> {
    id: actix_session::Session,
    req: HttpRequest,
    data: web::Data<State>,
    body: web::Json<T>,
}

pub struct UserRequest;

pub struct IdQueryParam<T: Model + Serialize> {
    id: i32,
    model: T,
}

/*
#[derive(FromRequest)]
pub struct ReqData<T> {
    path: web::Path<(String, String)>,
    query: web::Query<HashMap<String, String>>,
    json: web::Json<T>,
}
*/
