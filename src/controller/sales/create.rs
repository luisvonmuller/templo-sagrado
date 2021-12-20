/*
8888888b.  8888888888 .d8888b.  8888888 .d8888b. 88888888888 8888888888 8888888b.  8888888 888b    888  .d8888b.
888   Y88b 888       d88P  Y88b   888  d88P  Y88b    888     888        888   Y88b   888   8888b   888 d88P  Y88b
888    888 888       888    888   888  Y88b.         888     888        888    888   888   88888b  888 888    888
888   d88P 8888888   888          888   "Y888b.      888     8888888    888   d88P   888   888Y88b 888 888
8888888P"  888       888  88888   888      "Y88b.    888     888        8888888P"    888   888 Y88b888 888  88888
888 T88b   888       888    888   888        "888    888     888        888 T88b     888   888  Y88888 888    888
888  T88b  888       Y88b  d88P   888  Y88b  d88P    888     888        888  T88b    888   888   Y8888 Y88b  d88P
888   T88b 8888888888 "Y8888P88 8888888 "Y8888P"     888     8888888888 888   T88b 8888888 888    Y888  "Y8888P88
*/

/* Import estabilish connection from main */
use crate::establish_connection;

/* Lenient form imports */
use diesel::prelude::*;

/* Multipart Form */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use crate::User;

#[post("/new-sale", data = "<sale>")]
pub fn new_sale(user: User, content_type: &ContentType, sale: Data) {
    use crate::models::{NewProductList, NewSale};
    use crate::schema::{product_list, sale};

    //Lenient form imports
    use chrono::Utc;

    //Pass hashing

    //let user_id = cookies.get_private("user_id").unwrap();
    /* ------------------- CREATE A SALE ------------------- */
    /* Create sale with empty values, after we will fill the sale */
    /* Respective Sale inserting */
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();
    /* Payment source and user id */
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_id"));
    /* FOR SECURITY POURPOSES ????????? */
    options
        .allowed_fields
        .push(MultipartFormDataField::text("sale_real_value"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("sale_points_value"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("sale_payment_source"));

    options
        .allowed_fields
        .push(MultipartFormDataField::text("product_id"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("product_list_amount"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("product_list_use_points"));

    /* Fill the options */
    let form_data = MultipartFormData::parse(content_type, sale, options).unwrap();

    /* Prepare the data for diesel insert */
    //let user_id = form_data.texts.get("user_id").unwrap()[0].text.to_string();

    /* FOR SECURITY POURPOSES ????????? */
    let sale_real_value = form_data.texts.get("sale_real_value").unwrap()[0]
        .text
        .to_string();
    let _sale_points_value = form_data.texts.get("sale_points_value").unwrap()[0]
        .text
        .to_string();
    let sale_payment_source = form_data.texts.get("sale_payment_source").unwrap()[0]
        .text
        .to_string();

    /* Execute the query and get the sale_id */
    let sale_inserted_id: i32 = diesel::insert_into(sale::table)
        .values(NewSale {
            sale_date: Utc::now().naive_utc(),
            sale_real_value: Some(sale_real_value.parse::<f64>().unwrap()),
            sale_points_value: Some(0),
            user_id: user.user_id as i32, // alterar para o user_id
            sale_status: Some(1),
            sale_payment_source: Some(sale_payment_source),
        })
        .returning(sale::sale_id)
        .get_result(&establish_connection())
        .unwrap();

    /* ------------------- CREATE A PRODCT LIST ------------------- */
    /* Create sale with empty values, after we will fill the sale */
    /* First we declare what we will be accepting on this form */

    /* Prepare the data for diesel insert */
    let product_id = form_data.texts.get("product_id").unwrap()[0]
        .text
        .to_string();

    /* ------------------------------------- */
    /* HAVE TO ACCEPT A LIST OF PRODUCT_LIST */
    /* ------------------------------------- */
    /* Insert product_list on database */
    diesel::insert_into(product_list::table)
        .values(NewProductList {
            product_id: product_id.parse::<i32>().unwrap(),
            product_list_amount: 1, //product_list_amount.parse::<i32>().unwrap(),
            product_list_use_points: false, //use_points,
            sale_id: sale_inserted_id,
        })
        .execute(&establish_connection())
        .unwrap();


    crate::controller::payments::give_cesar_what_belongs_to_cesar_with_product_id(
        user.user_id as i32,
        product_id.parse::<i32>().unwrap(),
    );
}

#[post("/new-stripe-sale", data = "<sale>")]
pub fn new_stripe_sale(user: User, content_type: &ContentType, sale: Data) {
    use crate::models::{NewProductList, NewSale, NewStripePayment};
    use crate::schema::{product_list, sale, stripe_payment};

    //Lenient form imports
    use chrono::Utc;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    /* Payment and sale info */
    options.allowed_fields = vec![
        /* Sale/Payment info */
        MultipartFormDataField::text("sale_real_value"),
        MultipartFormDataField::text("sale_points_value"),
        MultipartFormDataField::text("sale_payment_source"),
        /* Product info */
        MultipartFormDataField::text("product_id"),
        /* Stripe info */
        MultipartFormDataField::text("stripe_source"),
    ];

    /* Fill the options */
    let form_data = MultipartFormData::parse(content_type, sale, options).unwrap();

    /* Each one of them return an option */
    /* Sale/Payment info */
    let sale_real_value = form_data.texts.get("sale_real_value").unwrap()[0]
        .text
        .to_string();
    let _sale_points_value = form_data.texts.get("sale_points_value").unwrap()[0]
        .text
        .to_string();
    let sale_payment_source = form_data.texts.get("sale_payment_source").unwrap()[0]
        .text
        .to_string();
    /* Product info */
    let product_id = form_data.texts.get("product_id").unwrap()[0]
        .text
        .to_string();
    /* Stripe info */
    let stripe_source = form_data.texts.get("stripe_source").unwrap()[0]
        .text
        .to_string();

    /* ------------------- CREATE A NEW SALE ------------------- */
    /* Execute the query and get the sale_id */
    let sale_inserted_id: i32 = diesel::insert_into(sale::table)
        .values(NewSale {
            sale_date: Utc::now().naive_utc(),
            sale_real_value: Some(sale_real_value.parse::<f64>().unwrap()),
            sale_points_value: Some(0),
            user_id: user.user_id as i32,
            sale_status: Some(0),
            sale_payment_source: Some(sale_payment_source),
        })
        .returning(sale::sale_id)
        .get_result(&establish_connection())
        .unwrap();

    /* ------------------- CREATE A PRODCT LIST ------------------- */

    /* ------------------------------------- */
    /* HAVE TO ACCEPT A LIST OF PRODUCT_LIST */
    /* ------------------------------------- */
    /* Insert product_list on database */
    diesel::insert_into(product_list::table)
        .values(NewProductList {
            product_id: product_id.parse::<i32>().unwrap(),
            product_list_amount: 1, //product_list_amount.parse::<i32>().unwrap(),
            product_list_use_points: false, //use_points,
            sale_id: sale_inserted_id,
        })
        .execute(&establish_connection())
        .unwrap();

    /* ------------------- CREATE A STRIPE PAYMENT ------------------- */

    /* Insert product_list on database */
    diesel::insert_into(stripe_payment::table)
        .values(NewStripePayment {
            stripe_payment_source: stripe_source,
            sale_id: sale_inserted_id,
        })
        .execute(&establish_connection())
        .unwrap();
}
