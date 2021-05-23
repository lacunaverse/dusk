use std::fmt::{write, Display};

use store::store_link;
use util::LinkRequest;
use views::{DeleteTemplate, IndexTemplate};

use actix_web::{
    dev::HttpResponseBuilder,
    get,
    http::{header, StatusCode},
    post,
    web::Json,
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
pub enum NewError {
    InvalidForm,
    AlreadyExists,
    SomethingWentWrong,
}

impl Display for NewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidForm => write!(f, "Invalid form information was sent"),
            Self::AlreadyExists => write!(f, "This link already exists!"),
            Self::SomethingWentWrong => write!(f, "Something went wrong on our end"),
        }
    }
}

impl actix_web::error::ResponseError for NewError {
    fn error_response(&self) -> HttpResponse {
        match self {
            NewError::InvalidForm => HttpResponse::BadRequest().finish(),
            NewError::AlreadyExists => HttpResponse::Conflict().finish(),
            NewError::SomethingWentWrong => HttpResponse::InternalServerError().finish(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            NewError::InvalidForm => StatusCode::BAD_REQUEST,
            NewError::AlreadyExists => StatusCode::CONFLICT,
            NewError::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// API endpoint for generating a new shortlink.
#[post("/new")]
pub async fn serve_new_api(data: Json<LinkRequest>) -> Result<impl Responder, NewError> {
    let form = data.into_inner();
    match store_link(&form) {
        Ok(()) => Ok(HttpResponse::Ok().json(NewLinkResponse {
            status: String::from("ok"),
            shortlink: String::from(form.id),
        })),
        Err(error) => match error {
            store::SaveError::AlreadyExists => Err(NewError::AlreadyExists),
            store::SaveError::OpenFailure => Err(NewError::SomethingWentWrong),
            store::SaveError::WriteFailure => Err(NewError::SomethingWentWrong),
            store::SaveError::Other => Err(NewError::SomethingWentWrong),
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
