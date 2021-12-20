/* Template */
use rocket_contrib::templates::Template;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Macros. */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;


/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/clerk");
    Template::render("pages/testimonials/list", &map)
}

#[get("/list")]
pub fn list(_administrative: AdminUser) -> Json<String> {
    use crate::models::Testimonials;
    use crate::schema::testimonials;

    let results = testimonials::table
        .select(testimonials::all_columns)
        .load::<Testimonials>(&crate::establish_connection())
        .expect("Erro while retrieving Testimonials");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

#[get("/change-status/<id>/<status>")]
pub fn change_status(_administrative: AdminUser, id: i32, status: bool) {
    use crate::schema::testimonials;

    diesel::update(testimonials::table.filter(testimonials::testimonials_id.eq(id)))
        .set(testimonials::testimonials_status.eq(!status))
        .execute(&crate::establish_connection())
        .unwrap();
}
