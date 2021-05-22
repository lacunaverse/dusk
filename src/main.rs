use api::{serve_root, serve_new, serve_delete};

use actix_files;
use actix_web::{get, middleware, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(serve_root)
            .service(serve_new)
            .service(serve_delete)
            .service(
                actix_files::Files::new("/static", "./static/dist").disable_content_disposition(),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
