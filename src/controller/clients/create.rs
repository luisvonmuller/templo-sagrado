/*
8888888b.  8888888888 .d8888b.  8888888 .d8888b. 88888888888 8888888888 8888888b.  8888888 888b    888  .d8888b.
888   Y88b 888       d88P  Y88b   888  d88P  Y88b    888     888        888   Y88b   888   8888b   888 d88P  Y88b
888    888 888       888    888   888  Y88b.         888     888        888    888   888   88888b  888 888    888
888   d88P 8888888   888          888   "Y888b.      888     8888888    888   d88P   888   888Y88b 888 888
8888888P"  888       888  88888   888      "Y88b.    888     888        8888888P"    888   888 Y88b888 888  88888
888 T88b   888       888    888   888        "888    888     888        888 T88b     888   888  Y88888 888    888
888  T88b  888       Y88b  d88P   888  Y88b  d88P    888     888        888  T88b    888   888   Y8888 Y88b  d88P
888   T88b 8888888888 "Y8888P88 8888888 "Y8888P"     888     8888888888 888   T88b 8888888 888    Y888  "Y8888P88
*

use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, FileField, TextField};
use rocket::Data;
use rocket::http::ContentType;



//Lenient form imports
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;




#[post("/facebook2", data = "<data>")]
pub fn new_facebook_client() {
    /* 
        Do something awesome to process the data from Facebook O'auth system
    */



    /* 
        call new_client to do the other stuff
        That means you're doing DRY
    */

    new_client( SOME DATA PARSED TO MATCH new_client PARAMS);

}


#[post("/new-client", data = "<client>")]
pub fn new_client(content_type: &ContentType, client: Data) {
    /* Importings structs an macros down here */
    use crate::models::{NewSysUser, NewPhone};
    use crate::schema::{sysuser, phone};
    
    use std::fs;

    /* Database connection */
    use crate::establish_connection;
    
    /* */    
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    /* Pass hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    /* Allowow parsing text stuff */


    /* 
        USERS ON THIS SYSTEM WILL NOT HAVE ADDRESS (WHY?)
        BECAUSE THE MOTHERF***** TUGA DON'T WANT
    
    */



    /* Inserting Sys User */
    let user_inserted_id: i32 = diesel::insert_into(sysuser::table)
        .values(NewSysUser {
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
            user_password: hasher.result_str(),
            user_birthdate: NaiveDate::from_ymd(
                birth_split[2].parse().unwrap(),
                birth_split[1].parse().unwrap(),
                birth_split[0].parse().unwrap(),
            ),
            user_genre: user_genre.to_string(),
            user_alias: Some(thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect()),
            user_newsletter: true,
            user_creation: Utc::now().naive_utc(),
            user_lasttimeonline: Some(Utc::now().naive_utc()),
            user_points: 0,
            user_balance: 0.00,
            user_type_id: 2,
            user_status: Some(true),
        })
        .returning(sysuser::user_id)
        .get_result(&establish_connection())
        .unwrap();
    


    /* Respective phone inserting */
    diesel::insert_into(phone::table)
    .values(NewPhone {
        phone_number: user_phone_number.to_string(),
        user_id: user_inserted_id,
        phone_type_id: 1,
    })
    .execute(&establish_connection())
    .unwrap();


}