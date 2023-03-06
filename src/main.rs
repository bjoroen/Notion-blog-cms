mod api;
mod html;
mod notion;
mod utils;

use actix_web::middleware;
use api::index::{get_index, get_style};
use dotenv::dotenv;

use crate::api::index::{get_blog_html_page, get_style_post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "no-cache")))
            .service(get_index)
            .service(get_style)
            .service(get_style_post)
            .service(get_blog_html_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
