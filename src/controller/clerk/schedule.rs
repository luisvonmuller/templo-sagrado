use super::*;

use crate::models::{ClerkSchedule, NewClerkSchedule};
use crate::schema::clerk_schedule;
use diesel::*;
use rocket::request::LenientForm;
/*
    Self recursive call, if theres no schedule, it will call self until theres a schedule, I do expect to not
    have an infinite loop going here so soon haha
*/

#[get("/<user_id>")]
pub fn get_clerk_schedule(_administrative: AdminUser, user_id: i32) -> Json<Vec<ClerkSchedule>> {
    let schedule: Vec<ClerkSchedule> = clerk_schedule::table
        .select(clerk_schedule::all_columns)
        .filter(clerk_schedule::clerk_schedule_user_id.eq(user_id))
        .load::<ClerkSchedule>(&crate::establish_connection())
        .expect("Whoops, we cannot retrieve a schedule...");

    if schedule.len() > 0 {
        /* Return this */
        Json(schedule)
    } else {
        diesel::insert_into(clerk_schedule::table)
            .values(NewClerkSchedule {
                clerk_schedule_user_id: user_id, /* sysuser (user_id) */
                /* Monday */
                clerk_schedule_mon: false,
                clerk_schedule_mon_init: "00:00".to_string(),
                clerk_schedule_mon_end: "23:59".to_string(),
                /* Tuesday */
                clerk_schedule_tue: false,
                clerk_schedule_tue_init: "00:00".to_string(),
                clerk_schedule_tue_end: "23:59".to_string(),
                /* Wednesday */
                clerk_schedule_wed: false,
                clerk_schedule_wed_init: "00:00".to_string(),
                clerk_schedule_wed_end: "23:59".to_string(),
                /* Thursday */
                clerk_schedule_thu: false,
                clerk_schedule_thu_init: "00:00".to_string(),
                clerk_schedule_thu_end: "23:59".to_string(),
                /* Friday */
                clerk_schedule_fri: false,
                clerk_schedule_fri_init: "00:00".to_string(),
                clerk_schedule_fri_end: "23:59".to_string(),
                /* Saturday */
                clerk_schedule_sat: false,
                clerk_schedule_sat_init: "00:00".to_string(),
                clerk_schedule_sat_end: "23:59".to_string(),
                /* Sunday */
                clerk_schedule_sun: false,
                clerk_schedule_sun_init: "00:00".to_string(),
                clerk_schedule_sun_end: "23:59".to_string(),
            })
            .execute(&crate::establish_connection())
            .expect("Whoops, we cannot insert a new schedule... See: ");

        get_clerk_schedule(_administrative, user_id)
    }
}

#[derive(Debug, FromForm, AsChangeset, Clone)]
#[table_name = "clerk_schedule"]
pub struct FormClerkSchedule {
    pub clerk_schedule_id: i32,
    pub clerk_schedule_user_id: i32, /* sysuser (user_id) */
    /* Monday */
    pub clerk_schedule_mon: bool,
    pub clerk_schedule_mon_init: String,
    pub clerk_schedule_mon_end: String,
    /* Tuesday */
    pub clerk_schedule_tue: bool,
    pub clerk_schedule_tue_init: String,
    pub clerk_schedule_tue_end: String,
    /* Wednesday */
    pub clerk_schedule_wed: bool,
    pub clerk_schedule_wed_init: String,
    pub clerk_schedule_wed_end: String,
    /* Thursday */
    pub clerk_schedule_thu: bool,
    pub clerk_schedule_thu_init: String,
    pub clerk_schedule_thu_end: String,
    /* Friday */
    pub clerk_schedule_fri: bool,
    pub clerk_schedule_fri_init: String,
    pub clerk_schedule_fri_end: String,
    /* Saturday */
    pub clerk_schedule_sat: bool,
    pub clerk_schedule_sat_init: String,
    pub clerk_schedule_sat_end: String,
    /* Sunday */
    pub clerk_schedule_sun: bool,
    pub clerk_schedule_sun_init: String,
    pub clerk_schedule_sun_end: String,
}

#[post("/set", data = "<form_data>")]
pub fn set_clerk_schedule(
    _administrative: AdminUser,
    form_data: LenientForm<FormClerkSchedule>,
) -> Json<bool> {
    diesel::update(
        clerk_schedule::table
            .filter(clerk_schedule::clerk_schedule_id.eq(form_data.clerk_schedule_id)),
    )
    .set(form_data.into_inner())
    .execute(&crate::establish_connection())
    .expect("Some shit happnd");

    Json(true)
}
