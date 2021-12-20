pub mod read;

/* Macros. */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Administrative Session */
use crate::AdminUser;

/* Our needed datastructure */
use crate::models::Intend;

/* Template */
use rocket_contrib::templates::Template;


/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    /* Hashmap that will store users our user list information */
    let mut context = std::collections::HashMap::new();
    context.insert("path", "/clients");
    Template::render("pages/intends/list", &context)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

