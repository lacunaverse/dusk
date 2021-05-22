use askama_actix::Template;
#[derive(Template)]
#[template(path = "../views/index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "../views/delete.html")]
pub struct DeleteTemplate {}

#[derive(Template)]
#[template(path = "../views/not-found.html")]
pub struct NotFound {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
