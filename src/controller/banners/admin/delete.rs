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

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Also, must have format type.
Nobody should acess this by text/html. */
#[get("/delete-banner/<banner_id>", format = "json", rank = 1)]
pub fn delete_banner(_administrative: AdminUser, banner_id: i32) -> Json<bool> {
    use crate::schema::banners;
    use diesel::prelude::*;

    diesel::delete(banners::table.filter(banners::banner_id.eq(banner_id)))
        .execute(&crate::establish_connection())
        .expect("Whoops, we can't delete this.");

    Json(true)
}
