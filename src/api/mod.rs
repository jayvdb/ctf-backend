use actix_web::web;

mod error;
mod teams;

pub use error::ApiError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").service(web::scope("/teams").configure(teams::config)));
}
