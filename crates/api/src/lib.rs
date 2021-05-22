use util::LinkRequest;
use store::store_link;
use views::{DeleteTemplate, IndexTemplate};

use actix_web::{get, post, web::Json, HttpResponse, Responder, Result};

/// Serves the root index page
#[get("/")]
pub async fn serve_root() -> impl Responder {
    IndexTemplate {}
}

/// API endpoint for generating a new shortlink.
#[post("/new")]
pub async fn serve_new_api(data: Json<LinkRequest>) -> impl Responder {
    "not implemented"
}

/// Serves the delete page
#[get("/delete")]
pub async fn serve_delete() -> impl Responder {
    DeleteTemplate {}
}

/// API endpoint for deleting a shortlink.
#[post("/delete")]
pub async fn serve_delete_api() -> impl Responder {
    "not implemented"
}

/// Serves a redirect link
#[get("/link/{id}")]
pub async fn serve_link() -> impl Responder {
    "not implemented"
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
