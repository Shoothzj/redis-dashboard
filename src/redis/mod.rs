mod keys;

use actix_web::{HttpResponse, web};

pub fn redis_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)))
        .service(web::resource("/keys").route(web::put().to(keys::put_key)).route(web::get().to(keys::get_key_list)))
        .service(web::resource("/keys/{key}").route(web::get().to(keys::get_key)).route(web::delete().to(keys::delete_key)));
    ;
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, Redis")
}
