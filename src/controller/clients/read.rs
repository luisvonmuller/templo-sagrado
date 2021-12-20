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

/* Macros. */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Administrative Session */
use crate::AdminUser;

/* Form */
use rocket::request::LenientForm;

/* Hash map and Datatables */
use rdatatables::*;

/* This one stands for our query data structure */
use crate::models::rdatatables::DataTablesSysUserListing;

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<OutcomeData<DataTablesSysUserListing>> {
    Json(datatables_query::<DataTablesSysUserListing>(
        Tables {
            origin: ("sysuser", "user_id"), /* From */
            fields: vec![
                "user_name",
                "user_balance",
                "user_bonus",
                "user_creation",
                "user_lasttimeonline",
                "user_status",
                "user_id",
            ], /* Fields to seek for */
            join_targets: None,             /* Join Targets explained over here */
            datatables_post_query: query.into_inner(), /* Incoming Query */
            query: None,                    /* Our builded query holder */
            condition: Some(vec![("AND", "user_type_id", "1")]),
        },
        crate::establish_connection(),
    ))
}

/* Data nmodeling to a good return */
use crate::models::{Address, Phone, SysUser};

#[get("/single/<user_id>")]
pub fn single(_administrative: AdminUser, user_id: i32) -> Json<Vec<(SysUser, Address, Phone)>> {
    use crate::schema::{address, phone, sysuser};

    let results = sysuser::table
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_id.eq(user_id))
        .load::<(SysUser, Address, Phone)>(&crate::establish_connection())
        .expect("Has no return from this. ");
    Json(results)
}

#[get("/balance/<user_id>")]
pub fn balance(_administrative: AdminUser, user_id: i32) -> Json<f64> {
    use crate::schema::sysuser;

    let results = sysuser::table
        .select(sysuser::user_balance)
        .filter(sysuser::user_id.eq(user_id))
        .load::<f64>(&crate::establish_connection())
        .expect("Shit happned while retrieving user minutes information.");

    Json(results[0])
}

#[get("/bonus/<user_id>")]
pub fn bonus(_administrative: AdminUser, user_id: i32) -> Json<f64> {
    use crate::schema::sysuser;

    let results = sysuser::table
        .select(sysuser::user_bonus)
        .filter(sysuser::user_id.eq(user_id))
        .load::<f64>(&crate::establish_connection())
        .expect("Shit happned while retrieving user minutes information.");

    Json(results[0])
}

/* That weird function import (the one that opens us to SQL_INJECTION attacks <3) */
use diesel::sql_query;

/* Importing the compatible data structure to this fucking query  🙋 */
use crate::models::BirthdateListing;

#[get("/get-birthdates/<days>")]
pub fn get_birthdates(_administrative: AdminUser, days: i32) -> Json<Vec<BirthdateListing>> {
    let results: Vec<BirthdateListing> = sql_query(format!(r#"
    select user_id, user_name, user_birthdate 
    from (
         select *, 
           (extract(year from age(user_birthdate)) + 1) *  interval '1 year' + user_birthdate "next_birth_day"
          from public.sysuser where user_type_id=1
    ) as users_with_upcoming_birth_days
    where next_birth_day  between now() and now() + '{} days' ORDER BY extract(day from user_birthdate) ASC"#, days))
    .load::<BirthdateListing>(&crate::establish_connection())
    .expect("Bad Query");

    Json(results)
}
