use api::{serve_delete, serve_delete_api, serve_link, serve_new_api, serve_root};

use actix_files;
use actix_web::{middleware, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(serve_root)
            .service(serve_new_api)
            .service(serve_delete)
            .service(serve_delete_api)
            .service(
                actix_files::Files::new("/static", "./static/dist").disable_content_disposition(),
            )
            .service(serve_link)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
