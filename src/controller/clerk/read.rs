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

/* Common db connection function */
use crate::establish_connection;

/* Macros. */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Hash map and Datatables */
use rdatatables::*;

/* Form */
use rocket::request::LenientForm;

use crate::models::rdatatables::{
    ClerksViewListing, ClerksViewListingClerkInfo, ClerksViewListingStatusClerk,
};

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<
    OutcomeData<(
        ClerksViewListing,
        ClerksViewListingStatusClerk,
        ClerksViewListingClerkInfo,
    )>,
> {
    Json(datatables_query::<(
        ClerksViewListing,
        ClerksViewListingStatusClerk,
        ClerksViewListingClerkInfo,
    )>(
        Tables {
            origin: ("sysuser", "user_id"), /* From */
            fields: vec![
                "clerk_info.clerk_image",
                "sysuser.user_name",
                "sysuser.user_uni",
                "sysuser.user_balance",
                "status_clerk.status",
                "sysuser.user_status",
                "sysuser.user_id",
            ], /* Fields to seek for */
            join_targets: Some(vec![
                ("inner", ("clerk_info", "user_id"), ("sysuser", "user_id")),
                (
                    "inner",
                    ("status_clerk", "clerk_id"),
                    ("sysuser", "user_id"),
                ),
            ]),
            datatables_post_query: query.into_inner(), /* Incoming Query parses to the desired struct. */
            query: None,
            condition: None, /* Our builded query holder */
        },
        crate::establish_connection(),
    ))
}

/* Every function that handles a rank  it's a func that holds no-allowed users to acess our data */
#[get("/list", format = "json", rank = 2)]
pub fn list_hack() -> Json<&'static str> {
    Json("You are not allowed to reach this content. Please, leave or dont't. You will not access it anyways.")
}

#[get("/single/<user_id>")]
pub fn single(_adminstrative: AdminUser, user_id: i32) -> Json<String> {
    use crate::models::{Address, ClerkBank, ClerkInfo, Phone, SysUser};
    use crate::schema::{address, clerk_bank, clerk_info, phone, sysuser};

    let results = sysuser::table
        .inner_join(clerk_info::table)
        .inner_join(clerk_bank::table)
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            clerk_info::all_columns,
            clerk_bank::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_id.eq(user_id))
        .load::<(SysUser, ClerkInfo, ClerkBank, Address, Phone)>(&establish_connection())
        .expect("Some shit happned while retrieving a single clerk!! <Panic at the Thread>");

    Json(serde_json::to_string(&results).unwrap())
}
