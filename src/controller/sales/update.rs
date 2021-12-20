/* Import estabilish connection from main */
use crate::establish_connection;

/* Lenient form imports */
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct SourceData {
    stripe_source: String,
    status: String,
    product_id: String,
}

#[post("/update-stripe-sale", format = "json", data = "<charge>")]
pub fn update_stripe_sale(charge: Json<SourceData>) {
    use crate::models::{Sale, StripePayment};
    use crate::schema::{sale, stripe_payment};

    use diesel::prelude::*;
    let _connection = establish_connection();

    /* Fill the options */

    let stripe_source = &charge.stripe_source;
    let status = &charge.status;
    let _product_id = &charge.product_id;

    let stripe_all = stripe_payment::table
        .select(stripe_payment::all_columns)
        .filter(stripe_payment::stripe_payment_source.eq(stripe_source))
        .load::<StripePayment>(&establish_connection())
        .expect("Some Error occured while parsing StripePayment values. Registered in logs.");

    let sale_all = sale::table
        .select(sale::all_columns)
        .filter(sale::sale_id.eq(stripe_all[0].sale_id))
        .load::<Sale>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    /* -------- UPDATE SALE STATUS-------- */

    diesel::update(sale::table.filter(sale::sale_id.eq(sale_all[0].sale_id)))
        .set(sale::sale_status.eq(status.parse::<i32>().unwrap()))
        .execute(&establish_connection())
        .unwrap();
        
    crate::controller::payments::give_cesar_what_belongs_to_cesar_with_product_id(
        sale_all[0].user_id as i32,
        _product_id.parse::<i32>().unwrap(),
    );
}
