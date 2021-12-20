/* Submodules declarations */
pub mod create;
pub mod delete;
pub mod read;
pub mod single;
pub mod update;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

/* Template */
use rocket_contrib::templates::Template;
#[get("/")]
pub fn index(_user: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/clerk");
    Template::render("pages/product/list", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}