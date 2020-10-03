use crate::{
    handlers::{self, admin, auth, index, record, user},
    middleware,
    state::{self, State},
};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_service::ServiceFactory;
use actix_web::{
    body, dev, get, http,
    middleware::{DefaultHeaders, Logger},
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use divdb::db::Db;
use serde::{Deserialize, Serialize};
use std::{net::TcpListener, sync::mpsc};

pub async fn run_api(listener: TcpListener) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let srv = HttpServer::new(move || {
        App::new()
            .data(state::state())
            .wrap(middleware::logger())
            .wrap(middleware::identity_service())
            .configure(handlers::routes)
    });
    srv.listen(listener)?.run().await?;
    Ok(())
}

pub fn spawn_api(listener: TcpListener, tx: mpsc::Sender<dev::Server>) -> std::io::Result<()> {
    let mut sys = actix_rt::System::new("api");
    let srv = HttpServer::new(move || {
        App::new()
            .data(state::state())
            .wrap(middleware::logger())
            .wrap(middleware::identity_service())
            .configure(handlers::routes)
    })
    .listen(listener)?
    .run();
    let _ = tx.send(srv.clone());
    sys.block_on(srv)
}

pub fn create_app(
    state: &State,
) -> App<
    impl ServiceFactory<
        Config = (),
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<body::Body>,
        Error = Error,
        InitError = (),
    >,
    body::Body,
> {
    App::new()
        .data(state::state())
        .wrap(middleware::cors().finish())
        .wrap(middleware::identity_service())
        .configure(handlers::routes)
}

#[derive(Serialize, Deserialize)]
pub struct TestEcho {
    num: i32,
    string: String,
}

#[get("/test")]
pub async fn test_route(req: HttpRequest, test: web::Json<TestEcho>) -> HttpResponse {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body(&test.string)
}