/* For JSON */
use serde::{Serialize, Deserialize};

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Macros and models required */
use crate::models::{NewSale, NewProductList, NewPaypalPayment};
use crate::schema::{sale, product_list, paypal_payment};

/* Macros from diesel */
use diesel::prelude::*;

/* Time stamp */
use chrono::Utc;

/* Paypal New Sale Data Structure */
#[derive(Serialize, Debug, Deserialize)]
pub struct PaypalSale {
	pub user_id: i32,
	pub product_id: i32,
    pub product_value: f64,
    pub reference_code: String,
}

/* Paypal Update Sale desired Json Structure */
#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateSale {
    pub reference_code: String,
}

#[post("/new-paypal-sale", format="json", data="<paypal_sale>")]
pub fn new_paypal_sale(paypal_sale: Json<PaypalSale>) -> () {
    /* ------------------- CREATE A NEW SALE ------------------- */
    /* Execute the query and get the sale_id */
    let sale_inserted_id: i32 = diesel::insert_into(sale::table)
        .values(NewSale {
            sale_date: Utc::now().naive_utc(),
            sale_real_value: Some(paypal_sale.product_value),
            sale_points_value: Some(0),
            user_id: paypal_sale.user_id,
            sale_status: Some(0),
            sale_payment_source: Some(paypal_sale.reference_code.to_owned()),
        })
        .returning(sale::sale_id)
        .get_result(&crate::establish_connection())
        .unwrap();

    /* ------------------- CREATE A PRODCT LIST ------------------- */
    /* Insert product_list on database */
    diesel::insert_into(product_list::table)
        .values(NewProductList {
            product_id: paypal_sale.product_id,
            product_list_amount: 1,
            product_list_use_points: false,
            sale_id: sale_inserted_id,
        })
        .execute(&crate::establish_connection())
        .unwrap();

    /* ---------------- REGISTER PAYPAL SALE ------------------- */
    /* Insert the reference row */
    diesel::insert_into(paypal_payment::table)
            .values(NewPaypalPayment { 
                paypal_payment_source_identifier: paypal_sale.reference_code.to_owned(),
                paypal_payment_sale_id: sale_inserted_id,
            })
            .execute(&crate::establish_connection())
            .unwrap();

    println!("{:?}, {}", paypal_sale, sale_inserted_id);
}

#[post("/update-paypal-sale", format="json", data="<paypal_sale>")]
pub fn update_paypal_sale(paypal_sale: Json<UpdateSale>) -> () { 
    /* Get Sale ID to retrieve desired values and others */
    let paypals_matching_ids = paypal_payment::table
                        .select(
                            paypal_payment::paypal_payment_sale_id
                         )
                        .filter(paypal_payment::paypal_payment_source_identifier.eq(paypal_sale.reference_code.to_owned()))
                        .load::<i32>(&crate::establish_connection())
                        .expect("We cannnot retrieve the desired id from paypal payments table");

    /* Get product ID - We cannot get it over joining cuz its not a defined PK */
    let product_matching_id = product_list::table 
                              .select(product_list::product_id)
                              .filter(product_list::sale_id.eq(paypals_matching_ids[0]))
                              .load::<i32>(&crate::establish_connection())
                              .expect("We cannot retrieve the product id");

    /* Get Sysuser(User_id) */
    let sysuser_user_id_matching_id = sale::table
                                        .select(sale::user_id)
                                        .filter(sale::sale_id.eq(paypals_matching_ids[0]))
                                        .load::<i32>(&crate::establish_connection())
                                        .expect("We cannot retrieve the user id");
    /* -------- UPDATE SALE STATUS-------- */
    diesel::update(sale::table.filter(sale::sale_id.eq(paypals_matching_ids[0])))
        .set(sale::sale_status.eq(Some(1)))
        .execute(&crate::establish_connection())
        .unwrap();
        
    /* We use product id cuz product can give also a bonus credit value */
    crate::controller::payments::give_cesar_what_belongs_to_cesar_with_product_id(
        sysuser_user_id_matching_id[0] as i32,
        product_matching_id[0] as i32,
    );

}