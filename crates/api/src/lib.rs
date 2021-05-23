use std::fmt::{write, Display};

use store::{get_link, store_link};
use util::LinkRequest;
use views::{DeleteTemplate, IndexTemplate};

use actix_web::{
    dev::HttpResponseBuilder,
    get,
    http::{header, StatusCode},
    post,
    web::{Json, Path},
    HttpResponse, Responder, ResponseError, Result,
};
use serde::{Deserialize, Serialize};

/// Serves the root index page
#[get("/")]
pub async fn serve_root() -> impl Responder {
    IndexTemplate {}
}

#[derive(Serialize, Deserialize, Clone)]
struct NewLinkResponse {
    status: String,
    shortlink: String,
}

#[derive(Debug)]
pub enum Error {
    InvalidForm,
    AlreadyExists,
    SomethingWentWrong,
    NotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidForm => write!(f, "Invalid form information was sent"),
            Self::AlreadyExists => write!(f, "This link already exists!"),
            Self::SomethingWentWrong => write!(f, "Something went wrong on our end"),
            Self::NotFound => write!(f, "The requested resource could not found."),
        }
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InvalidForm => HttpResponse::BadRequest().finish(),
            Error::AlreadyExists => HttpResponse::Conflict().finish(),
            Error::SomethingWentWrong => HttpResponse::InternalServerError().finish(),
            Error::NotFound => HttpResponse::NotFound().finish(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidForm => StatusCode::BAD_REQUEST,
            Error::AlreadyExists => StatusCode::CONFLICT,
            Error::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

/// API endpoint for generating a new shortlink.
#[post("/new")]
pub async fn serve_new_api(data: Json<LinkRequest>) -> Result<impl Responder, Error> {
    let form = data.into_inner();
    match store_link(&form) {
        Ok(()) => Ok(HttpResponse::Ok().json(NewLinkResponse {
            status: String::from("ok"),
            shortlink: String::from(form.id),
        })),
        Err(error) => match error {
            store::SaveError::AlreadyExists => Err(Error::AlreadyExists),
            store::SaveError::OpenFailure => Err(Error::SomethingWentWrong),
            store::SaveError::WriteFailure => Err(Error::SomethingWentWrong),
            store::SaveError::Other => Err(Error::SomethingWentWrong),
        },
    }
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

#[derive(Deserialize)]
struct ShortLink {
    pub id: String,
}

/// Serves a redirect link
#[get("/link/{id}")]
pub async fn serve_link(path: Path<ShortLink>) -> Result<impl Responder, Error> {
    match get_link(&path.id) {
        Ok(url) => match url.len() {
            0 => Err(Error::SomethingWentWrong),
            _ => Ok(HttpResponse::TemporaryRedirect()
                .set_header("Location", url)
                .finish()),
        },
        Err(error) => match error {
            store::GetError::NotFound => Err(Error::NotFound),
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
