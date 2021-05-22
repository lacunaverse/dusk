use askama_actix::Template;
#[derive(Template)]
#[template(path = "../views/index.html")]
pub struct IndexTemplate {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
