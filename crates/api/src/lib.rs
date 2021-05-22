use actix_web::{get, middleware, post, App, HttpServer, Responder};
use views::{DeleteTemplate, IndexTemplate};

/// Serves the root index page
#[get("/")]
pub async fn serve_root() -> impl Responder {
    IndexTemplate {}
}

/// API endpoint for generating a new shortlink.
#[post("/new")]
pub async fn serve_new_api() -> impl Responder {
    "not implemented"
}

/// Serves the delete page
#[get("/delete")]
pub async fn serve_delete() -> impl Responder {
    DeleteTemplate{}
}

/// API endpoint for deleting a shortlink.
#[post("/delete")]
pub async fn serve_delete_api() -> impl Responder {
    "not implemented"
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
