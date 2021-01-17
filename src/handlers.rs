pub mod admin;
pub mod graphql;
pub mod auth;
pub mod record;
pub mod user;
pub mod item;
pub mod group;
pub mod upload;
pub mod public;
pub mod fact;

use actix_session::Session;
use actix_web::{ App,
    web, web::ServiceConfig, HttpRequest, HttpResponse, Responder,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        // .service(test_service())
        // .service(public::public_routes())
        // .service(web::scope("/api")
            .service(user::uid_routes())
            .service(user::username_routes())
            .service(auth::routes())
            .service(record::base_routes())
            .service(record::user_record_routes())
            .service(item::base_routes())
            .service(item::user_item_routes())
            .service(admin::routes())
            .service(fact::routes());
        // );
}

pub(crate) fn test_service() ->  actix_web::Resource {
    web::resource("/test/{test}")
        .route(web::get().to(|test: web::Path<String>| {
            HttpResponse::Ok().body(format!("GET /test/{}", test))
        }))
        .route(web::post().to(|| HttpResponse::Ok().body("")))
}

pub async fn route_404(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("No route here")
}

