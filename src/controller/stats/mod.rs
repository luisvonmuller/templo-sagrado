
/* Mods by weeks */
pub mod weeks;
/* Mods by days */
pub mod days;

use crate::models::{Sum, NewSalesMonthsBeforeData, NewSalesMonthsBeforeDataResult};

/* Common db connection function */
use crate::establish_connection;

/* Json type response (must have)  */
use rocket_contrib::json::Json;

/* Macros and other stuffs */
use diesel::*;
use rdatatables::Count;
/* Importing User struct of our session handler */
use crate::AdminUser;
use chrono::Utc;

#[get("/minutes-voice")]
pub fn minutes_voice(_administrative: AdminUser) -> Json<i32> {
    use crate::schema::global_states;

    let value: i32 = global_states::table.select(
        global_states::voice_minutes
    ).load::<i32>(&establish_connection())
    .expect("Nothing wrong happned")[0];

    Json(value)
}

#[get("/total-clients")]
pub fn total_clients(_administrative: AdminUser) -> Json<String> {
    use crate::schema::sysuser;
    use diesel::dsl::count;

    let results = sysuser::table
        .select(count(sysuser::user_id))
        .filter(sysuser::user_type_id.eq(1))
		.load::<i64>(&establish_connection())
		.expect("Some shit happned while retrieving the statics of all Clients!!! <Panic at the Disco> ops, thread!");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/total-active-clients")]
pub fn total_active_clients(_administrative: AdminUser) -> Json<String> {
    use crate::schema::sysuser;
    use diesel::dsl::count;

    let results = sysuser::table
        .select(count(sysuser::user_id))
        .filter(sysuser::user_type_id.eq(1))
        .filter(sysuser::user_status.eq(true))
        .load::<i64>(&establish_connection())
		.expect("Some shit happned while retrieving the statics of all Clients!!! <Panic at the Disco> ops, thread!");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/total-clerks")]
pub fn total_clerks(_administrative: AdminUser) -> Json<String> {
    use crate::schema::sysuser;
    use diesel::dsl::count;

    let results = sysuser::table
        .select(count(sysuser::user_id))
        .filter(sysuser::user_type_id.eq(2))
		.load::<i64>(&establish_connection())
		.expect("Some shit happned while retrieving the statics of all Clients!!! <Panic at the Disco> ops, thread!");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/total-active-clerks")]
pub fn total_active_clerks(_administrative: AdminUser) -> Json<String> {
    use crate::schema::sysuser;
    use diesel::dsl::count;

    let results = sysuser::table
        .select(count(sysuser::user_id))
        .filter(sysuser::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .load::<i64>(&establish_connection())
		.expect("Some shit happned while retrieving the statics of all Clients!!! <Panic at the Disco> ops, thread!");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/new-clients/<minus_month_indentifier>")]
pub fn new_clients(_administrative: AdminUser, minus_month_indentifier: i32) -> Json<Count> {
    let count: Count = sql_query(format!(
        "SELECT count(*) FROM sysuser
        WHERE user_creation >= date_trunc('month', current_date - interval '{}' month)
        AND user_creation < date_trunc('month', current_date - interval '{}' month)",
        minus_month_indentifier,
        (minus_month_indentifier - 1)
    ))
    .load::<Count>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}

/*
*   Total Amount of money (sum), the total amount of sales (count), and the number of the month in the year.
*   Stands for the total amount we have sold by the month in the interval
*   @num_weeks_before = the number of months before this month. 
*/
#[get("/new-sales-months-before/<num_months_before>")]
pub fn new_sales_months(_administrative: AdminUser, num_months_before: u16) -> Json<NewSalesMonthsBeforeData> {
    /* Sum of the values of sales in the week */
    let mut months_sum: Vec<f64> = Vec::new();
    /* Count of the sales in the week */
    let mut months_count: Vec<i64> = Vec::new();
    /* Index of the week to weekAlias frontend vector */
    let mut months: Vec<i32> = Vec::new();

    if num_months_before > 0 && num_months_before <= 36 {
        /* now */
        let the_date = Utc::now();
        //let mut date_for_month
        for i in 0..=num_months_before {
            match sql_query(format!(
                "SELECT sum(sale_real_value), count(*), CAST (EXTRACT(MONTH FROM date_trunc('day', date_trunc('month', to_date('{}', 'YYYY-MM-DD')) - interval '{}' month)) as INTEGER) as month_index
                    FROM sale
                    WHERE sale_date >= date_trunc('day', date_trunc('month', to_date('{}', 'YYYY-MM-DD')) - interval '{}' month)
                        and sale_date < date_trunc('day', date_trunc('month', to_date('{}', 'YYYY-MM-DD')) - interval '{}' month) AND sale_status = 1",
                    the_date.date(),
                    i,
                    the_date.date(),
                    i,
                    the_date.date(),
                    i as i32 -1
                ))
                /* Loading the Sum of sales and the total count of sales */
                .load::<NewSalesMonthsBeforeDataResult>(&crate::establish_connection()) {
                    Ok(iter_res) => { 
                        /* If have results */
                        if iter_res.len() > 0 {
                            /* Option tratament for sum */
                            months_sum.push(match iter_res[0].sum {
                                Some(s) => { s }
                                None => { 0.0 }
                            }); 
                            /* Option tratament for count */
                            months_count.push(match iter_res[0].count {
                                Some(c) => { c }
                                None => { 0 }
                            });
                            /* Option tratament for month_index */
                            months.push(match iter_res[0].month_index {
                                Some(m) => { m }
                                None => { 0 }
                            });
                        }
                    }
                    /* If any error occurred, result sum and count is 0 */
                    Err(_) => { 
                        months_sum.push(0.0);
                        months_count.push(0);
                        months.push(0);
                    }
            }
        }
        months_sum.reverse();
        months_count.reverse();
        months.reverse();
    }
    Json(NewSalesMonthsBeforeData {
        sum: months_sum,
        count: months_count,
        month: months
    })
}

/*
    Total amount (int)
    Stands for count of realized sales on a specifyed month
*/

#[get("/new-sales/<minus_month_indentifier>")]
pub fn new_sales(_administrative: AdminUser, minus_month_indentifier: i32) -> Json<Count> {
    let count: Count = sql_query(format!(
        "SELECT count(*)
    FROM sale
    WHERE sale_date >= date_trunc('month', current_date - interval '{}' month)
      and sale_date < date_trunc('month', current_date - interval '{}' month) AND sale_status = 1",
        minus_month_indentifier,
        (minus_month_indentifier - 1)
    ))
    .load::<Count>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}

/*
    Total Amount as money
    Stands for the total amount the we have sold this month
*/
#[get("/new-sales-values/<minus_month_indentifier>")]
pub fn new_sales_values(_administrative: AdminUser, minus_month_indentifier: i32) -> Json<Sum> {
    let count: Sum = sql_query(format!(
        "SELECT sum(sale_real_value)
    FROM sale
    WHERE sale_date >= date_trunc('month', current_date - interval '{}' month)
      and sale_date < date_trunc('month', current_date - interval '{}' month) AND sale_status = 1",
        minus_month_indentifier,
        (minus_month_indentifier - 1)
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}
