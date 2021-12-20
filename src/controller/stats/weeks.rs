use chrono::{Datelike, Utc, Duration};
use rocket_contrib::json::Json;
use crate::models::{NewSalesWeeksBeforeData, NewSalesWeeksBeforeDataResult};
use diesel::dsl::sql_query;
use crate::diesel::RunQueryDsl;

/* Importing User struct of our session handler */
use crate::AdminUser;

/*
*    Total Amount of money (sum), the total amount of sales (count), and the index of the week 
*    Stands for the total amount we have sold by the week in the interval
*    @num_weeks_before = the number of weeks before this week. Weeks as start on Monday
*/

#[get("/new-sales-weeks-before/<num_weeks_before>")]
pub fn new_sales_week(_administrative: AdminUser, num_weeks_before: u16) -> Json<NewSalesWeeksBeforeData> {
    /* Sum of the values of sales in the week */
    let mut weeks_sum: Vec<f64> = Vec::new();
    /* Count of the sales in the week */
    let mut weeks_count: Vec<i64> = Vec::new();
    /* Index of the week to weekAlias frontend vector */
    let mut week_alias: Vec<u32> = Vec::new();
    /* now */
    let the_date_now = Utc::now();
    /* next saturday */
    let week_sunday = the_date_now.checked_add_signed(Duration::days((7 - the_date_now.weekday().num_days_from_monday()).into())).unwrap();
    /* filtering a secure interval */
    if num_weeks_before > 0 && num_weeks_before <= 52 {
        /* The limit date is Saturday the current week */
        let mut last_week_sunday = week_sunday;
        /* week in weeks */
        for i in 1..=num_weeks_before {
            match sql_query(format!(
                "SELECT sum(sale_real_value), count(*)
                FROM sale
                WHERE sale_date < date_trunc('day', to_date('{}', 'YYYY-MM-DD'))
                    and sale_date >= date_trunc('day', to_date('{}', 'YYYY-MM-DD')) AND sale_status = 1", last_week_sunday.date(), last_week_sunday.checked_sub_signed(Duration::weeks(1)).unwrap().date()
                ))
                /* Loading the Sum of sales and the total count of sales */
                .load::<NewSalesWeeksBeforeDataResult>(&crate::establish_connection()) {
                    Ok(iter_res) => { 
                        /* If have results */
                        if iter_res.len() > 0 {
                            /* Option tratament for sum */
                            weeks_sum.push(match iter_res[0].sum {
                                Some(s) => { s }
                                None => { 0.0 }
                            }); 
                            /* Option tratament for count */
                            weeks_count.push(match iter_res[0].count {
                                Some(c) => { c }
                                None => { 0 }
                            });
                        }
                    }
                    /* If any error occurred, result sum and count is 0 */
                    Err(_) => { 
                        weeks_sum.push(0.0);
                        weeks_count.push(0);
                    }
            }
            last_week_sunday = week_sunday.checked_sub_signed(Duration::weeks(i.into())).unwrap();
            week_alias.push((i-1).into());
        }
        weeks_sum.reverse();
        weeks_count.reverse();
        week_alias.reverse();
    }
    /* Return the vec with the total sales by week and the vec with total sales count by week */
    Json(NewSalesWeeksBeforeData {
        sum: weeks_sum,
        count: weeks_count,
        week_alias: week_alias
    })
}
/*
    Total amount (int)
    Stands for count of realized sales on a specifyed month
*/
/*#[get("/new-sales-week/<minus_week_identifier>")]
pub fn new_sales(_administrative: AdminUser, minus_week_identifier: i32) -> Json<Count> {
    let count: Count = sql_query(format!(
        "SELECT count(*)
    FROM sale
    WHERE sale_date <= date_trunc('day', current_date - interval '{}' day)
      and sale_date >= date_trunc('day', current_date - interval '{}' day) AND sale_status = 1",
      minus_week_identifier,
        (minus_week_identifier + 6)
    ))
    .load::<Count>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}
*/
/*
    Total Amount as money
    Stands for the total amount the we have sold this month
*/
/*use diesel::*;

#[get("/new-sales-values-week/<minus_week_identifier>")]
pub fn new_sales_values(_administrative: AdminUser, minus_week_identifier: i32) -> Json<Sum> {
    let count: Sum = sql_query(format!(
        "SELECT sum(sale_real_value)
    FROM sale
    WHERE sale_date <= date_trunc('day', current_date - interval '{}' day)
      and sale_date >= date_trunc('day', current_date - interval '{}' day) AND sale_status = 1",
      minus_week_identifier,
        (minus_week_identifier + 6)
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}*/