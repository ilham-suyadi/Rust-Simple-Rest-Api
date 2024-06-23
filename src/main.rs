use actix_web::{App, HttpServer};
mod service;
mod data;
use service::{get_blogs, post_blog, get_blog, update_blog, delete_blog};
use data::init_state;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Rest Api Service");
    let app_state = init_state();
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_blogs)
            .service(post_blog)
            .service(get_blog)
            .service(update_blog)
            .service(delete_blog)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}