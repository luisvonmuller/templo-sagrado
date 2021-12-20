/*
888      8888888 .d8888b. 88888888888             d8888 888b    888 8888888b.       8888888b.  8888888888        d8888 8888888b.
888        888  d88P  Y88b    888                d88888 8888b   888 888  "Y88b      888   Y88b 888              d88888 888  "Y88b
888        888  Y88b.         888               d88P888 88888b  888 888    888      888    888 888             d88P888 888    888
888        888   "Y888b.      888              d88P 888 888Y88b 888 888    888      888   d88P 8888888        d88P 888 888    888
888        888      "Y88b.    888             d88P  888 888 Y88b888 888    888      8888888P"  888           d88P  888 888    888
888        888        "888    888            d88P   888 888  Y88888 888    888      888 T88b   888          d88P   888 888    888
888        888  Y88b  d88P    888           d8888888888 888   Y8888 888  .d88P      888  T88b  888         d8888888888 888  .d88P
88888888 8888888 "Y8888P"     888          d88P     888 888    Y888 8888888P"       888   T88b 8888888888 d88P     888 8888888P"
 */

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Hash map and Datatables */
use rdatatables::*;

/* Form */
use crate::AdminUser;
use rocket::request::LenientForm;

/* This one stands for our query data structure */
use crate::models::rdatatables::DataTablesBannerListing;

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<OutcomeData<DataTablesBannerListing>> {
    Json(datatables_query::<DataTablesBannerListing>(
        Tables {
            origin: ("banners", "banner_id"), /* From */
            fields: vec![
                "banner_creation_date",
                "banner_mobile",
                "banner_desktop",
                "banner_id",
            ], /* Fields to seek for */
            join_targets: None,               /* Join Targets explained over here */
            datatables_post_query: query.into_inner(), /* Incoming Query */
            query: None,                      /* Our builded query holder */
            condition: None,
        },
        crate::establish_connection(),
    ))
}

/* Every function that handles a rank  it's a func that holds no-allowed users to access our data */
#[get("/list", format = "json", rank = 2)]
pub fn list_hack() -> Json<&'static str> {
    Json("You are not allowed to reach this content. Please, leave or dont't. You will not access it anyways.")
}
