use actix_web::web;

pub mod google_oauth;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api")
          .configure(google_oauth::routes)
    );
}