use chrono::{Datelike, Utc, Duration};
use rocket_contrib::json::Json;
use crate::models::{NewSalesDaysBeforeData, NewSalesDaysBeforeDataResult};
//use rdatatables::Count;
use diesel::RunQueryDsl;
use diesel::dsl::sql_query;

/* Importing User struct of our session handler */
use crate::AdminUser;

/*
*    Total Amount of money (sum), the total amount of sales (count), and the index of a weekday in the week
*    Stands for the total amount we have sold by the day in the interval
*    @num_days_before = the number of days before this day. 
*/

#[get("/new-sales-days-before/<num_days_before>")]
pub fn new_sales_days(_administrative: AdminUser, num_days_before: u16) -> Json<NewSalesDaysBeforeData> {
    let mut days_sum: Vec<f64> = Vec::new();
    let mut days_count: Vec<i64> = Vec::new();
    let mut week_days: Vec<u32> = Vec::new();
    
    if num_days_before > 0 && num_days_before <= 365 {
        /* now */
        let mut the_date = Utc::now();

        for _i in 0..=num_days_before {
            match sql_query(format!(
                "SELECT sum(sale_real_value), count(*)
                    FROM sale
                    WHERE sale_date >= date_trunc('day', date_trunc('day', to_date('{}', 'YYYY-MM-DD')) - interval '0' day)
                        and sale_date < date_trunc('day', date_trunc('day', to_date('{}', 'YYYY-MM-DD')) - interval '-1' day) AND sale_status = 1",
                    the_date.date(),
                    the_date.date()
                ))
                /* Loading the Sum of sales and the total count of sales */
                .load::<NewSalesDaysBeforeDataResult>(&crate::establish_connection()) {
                    Ok(iter_res) => { 
                        /* If have results */
                        if iter_res.len() > 0 {
                            /* Option tratament for sum */
                            days_sum.push(match iter_res[0].sum {
                                Some(s) => { s }
                                None => { 0.0 }
                            }); 
                            /* Option tratament for count */
                            days_count.push(match iter_res[0].count {
                                Some(c) => { c }
                                None => { 0 }
                            });
                        }
                    }
                    /* If any error occurred, result sum and count is 0 */
                    Err(_) => { 
                        days_sum.push(0.0);
                        days_count.push(0);
                    }
                }
            /* Getting the index to name of the day of the week */
            week_days.push(the_date.weekday().num_days_from_sunday());
            /* Decrementing the day in one*/
            the_date = the_date.checked_sub_signed(Duration::days(1)).unwrap();
        }
    }
    
    days_sum.reverse();
    days_count.reverse();
    week_days.reverse();

    Json(NewSalesDaysBeforeData {
        sum: days_sum,
        count: days_count,
        week_day: week_days,
    })
}
//use super::*;

/*
    Total amount (int)
    Stands for count of realized sales on a specifyed month
*/
/*#[get("/new-sales-days/<minus_day_indentifier>")]
pub fn new_sales(minus_day_indentifier: i32) -> Json<Count> {
    let count: Count = sql_query(format!(
        "SELECT count(*)
    FROM sale
    WHERE sale_date >= date_trunc('day', current_date - interval '{}' day)
      and sale_date < date_trunc('day', current_date - interval '{}' day) AND sale_status = 1",
        minus_day_indentifier,
        (minus_day_indentifier - 1)
    ))
    .load::<Count>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}*/

/*
    Total Amount as money
    Stands for the total amount the we have sold this month
*/
/*#[get("/new-sales-values-days/<minus_day_indentifier>")]
pub fn new_sales_values(minus_day_indentifier: i32) -> Json<Sum> {
    let count: Sum = sql_query(format!(
        "SELECT sum(sale_real_value)
        FROM sale
        WHERE sale_date >= date_trunc('day', current_date - interval '{}' day)
      and sale_date < date_trunc('day', current_date - interval '{}' day) AND sale_status = 1",
      minus_day_indentifier,
        (minus_day_indentifier - 1)
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}*/
