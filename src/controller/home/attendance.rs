/* Import imports from super module */
use super::*;

/* Import the module */
use crate::models::{ClerkTime, StatusClerk};

#[get("/atendimento")]
pub fn clerk_attendance(user: User) -> Template {
    use crate::models::{ClerkInfo, SysUser, UserType};
    use crate::schema::{call_email, clerk_info, status_clerk, sysuser, user_type};
    let mut context = HashMap::new();

    let self_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .inner_join(status_clerk::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .order(clerk_info::clerk_info_priority.desc())
        .order(status_clerk::status.desc())
        .limit(1)
        .load::<(SysUser, UserType, ClerkInfo)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let mails: Vec<(i32, String)> = call_email::table
        .inner_join(sysuser::table)
        .select((call_email::call_email_id, sysuser::user_name))
        .filter(call_email::clerk_id.eq(user.user_id as i32))
        .filter(call_email::call_email_response_body.is_null())
        .load::<(i32, String)>(&establish_connection())
        .expect("Shit happens");

    context.insert("self_data", (mails, self_data));
    Template::render("clerk/index", &context)
}

#[get("/my-last-status")]
pub fn my_last_status(user: User) -> Json<Vec<StatusClerk>> {
    use crate::schema::status_clerk;

    Json(
        status_clerk::table
            .select(status_clerk::all_columns)
            .filter(status_clerk::clerk_id.eq(user.user_id as i32))
            .load::<StatusClerk>(&crate::establish_connection())
            .expect("No clerk here"),
    )
}

#[get("/register-event-by-clerk/<num_status>")]
pub fn register_event_by_clerk(user: User, num_status: i32) {
    use crate::models::NewClerkTime;
    use crate::schema::clerk_time;
    use chrono::Utc;

    diesel::insert_into(clerk_time::table)
        .values(NewClerkTime {
            clerk_time_clerk_id: user.user_id as i32,
            clerk_time_date: Utc::now().naive_utc(),
            clerk_time_event_type: num_status as i32,
        })
        .execute(&crate::establish_connection())
        .expect("Something wrong has occurred: ");
}

#[get("/my-behaviour-on-a-day/<day>")]
pub fn my_behaviour_on_a_day(user: User, day: i32) -> Json<Vec<ClerkTime>> {
    use diesel::dsl::*;

    Json(
        sql_query(format!(
            "SELECT * FROM clerk_time WHERE CAST(clerk_time_date AS DATE) = date_trunc('day', current_date - {}) AND clerk_time_clerk_id={} ORDER BY clerk_time_id ASC",
            day,
            user.user_id as i32,
        ))
        .load::<ClerkTime>(&crate::establish_connection())
        .expect("Query failed"),
    )
}

use crate::models::{Day, Sum};

#[get("/text-transactions-data")]
pub fn text_transactions_data(user: User) -> Json<Vec<(Day, Sum)>> {
    use diesel::dsl::*;

    let data: Vec<(Day, Sum)> = sql_query(format!(
        r#"
        SELECT date_trunc('day', text_chat_transaction_creation) "day",
        SUM(text_chat_transaction_value_pay_off)
        FROM text_chat_transaction
        WHERE
            CAST(text_chat_transaction_creation as DATE) >= date_trunc('day', current_date - 60) 
            AND
            text_chat_transaction_clerk_id={}
        GROUP BY "day" ORDER BY "day" DESC LIMIT 60
    "#,
        user.user_id as i32,
    ))
    .load::<(Day, Sum)>(&crate::establish_connection())
    .expect("Query failed");

    Json(data)
}

#[get("/voice-transactions-data")]
pub fn voice_transactions_data(user: User) -> Json<Vec<(Day, Sum)>> {
    use diesel::dsl::*;

    let data: Vec<(Day, Sum)> = sql_query(format!(
        r#"
        SELECT date_trunc('day', voice_chat_transaction_creation) "day",
        SUM(voice_chat_transaction_value_pay_off)
        FROM voice_chat_transaction
        WHERE
            CAST(voice_chat_transaction_creation as DATE) >= date_trunc('day', current_date - 60) 
            AND
            voice_chat_transaction_clerk_id={}
        GROUP BY "day" ORDER BY "day" DESC LIMIT 60
    "#,
        user.user_id as i32,
    ))
    .load::<(Day, Sum)>(&crate::establish_connection())
    .expect("Query failed");

    Json(data)
}

#[get("/earned-since-begun")]
pub fn earned_since_begun(user: User) -> Json<f64> {
    use diesel::dsl::*;

    let earned_text : Sum = sql_query(format!("SELECT Sum(text_chat_transaction_value_pay_off) from text_chat_transaction where text_chat_transaction_clerk_id={}", user.user_id as i32 ))
                            .load::<Sum>(&crate::establish_connection())
                            .expect("Query failed")
                            .pop()
                            .expect("No rows");

    let earned_voice : Sum = sql_query(format!("SELECT Sum(voice_chat_transaction_value_pay_off) from voice_chat_transaction where voice_chat_transaction_clerk_id={}", user.user_id as i32 ))
                            .load::<Sum>(&crate::establish_connection())
                            .expect("Query failed")
                            .pop()
                            .expect("No rows");

    Json(earned_voice.sum.unwrap_or(0.0) + earned_text.sum.unwrap_or(0.0))
}
