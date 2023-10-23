mod jokes;

use actix_web::{web, HttpResponse};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(web::scope("/jokes").configure(jokes::init));
    config.route(
        "/status",
        web::get().to(|| async { HttpResponse::Ok().json(serde_json::json!({"ok":true})) }),
    );
}
