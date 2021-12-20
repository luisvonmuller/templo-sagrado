/*

8888888b.  8888888888 888      8888888888 88888888888 8888888888
888  "Y88b 888        888      888            888     888
888    888 888        888      888            888     888
888    888 8888888    888      8888888        888     8888888
888    888 888        888      888            888     888
888    888 888        888      888            888     888
888  .d88P 888        888      888            888     888
8888888P"  8888888888 88888888 8888888888     888     8888888888

 */

/* Common db connection function */
use crate::establish_connection;

/* Macros. */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Importing User struct of our session handler */
use crate::User;


/* Also, must have format type.
Nobody should acess this by text/html. */
#[get("/delete/<product_id>", format = "json", rank = 1)]
pub fn delete_product(_user: User, product_id: i32) -> Json<String> {
    use crate::schema::product;

    let results = diesel::update(product::table.filter(product::product_id.eq(product_id)))
    .set(product::product_is_active.eq(false))
    .execute(&establish_connection())
    .unwrap();

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/delete", format = "json", rank = 2)]
pub fn list_hack() -> Json<&'static str> {
    Json("You are not allowed to reach this content. Please, leave or dont't. You will not access it anyways.")
}
