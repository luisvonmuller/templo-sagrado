/*
8888888b.  8888888888 .d8888b.  8888888 .d8888b. 88888888888 8888888888 8888888b.  8888888 888b    888  .d8888b.
888   Y88b 888       d88P  Y88b   888  d88P  Y88b    888     888        888   Y88b   888   8888b   888 d88P  Y88b
888    888 888       888    888   888  Y88b.         888     888        888    888   888   88888b  888 888    888
888   d88P 8888888   888          888   "Y888b.      888     8888888    888   d88P   888   888Y88b 888 888
8888888P"  888       888  88888   888      "Y88b.    888     888        8888888P"    888   888 Y88b888 888  88888
888 T88b   888       888    888   888        "888    888     888        888 T88b     888   888  Y88888 888    888
888  T88b  888       Y88b  d88P   888  Y88b  d88P    888     888        888  T88b    888   888   Y8888 Y88b  d88P
888   T88b 8888888888 "Y8888P88 8888888 "Y8888P"     888     8888888888 888   T88b 8888888 888    Y888  "Y8888P88

ON A CRUD THE ELEMENT IS: CREATE (óbvio.) ;D
*/
//Import estabilish connection from main
use crate::establish_connection;

//Lenient form imports
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use rocket::request::LenientForm;

//Pass hashing
use crypto::digest::Digest;
use crypto::sha2::Sha512;

use rocket::http::{Cookie, Cookies};

/* Json response */
use rocket_contrib::json::Json;

//Form struct (must match to what is beeing posted) - Do not need to match the struct of the
// NewUser (Cuz form is diff of a Table Structure, both have some and dont some fields. )
#[derive(FromForm, Debug)]
pub struct FormNewUser {
    pub user_name: String,
    pub user_email: String,
    pub user_birthdate: String,
    pub user_genre: String,
    pub user_phone: String,
    pub user_zip_code: String,
    pub user_city: String,
    pub user_street: String,
    pub user_addr_num: String,
    pub user_alias: String,
    pub user_pass: String,
    pub user_uni: String,
    pub user_news_letter: Option<String>,
}

/*
    Here user _ have an underline for easy identifying thats
    a temporary and a can fail post (when not matching struct)
*/
#[post("/new-user", data = "<_user>")]
pub fn new_user(mut cookies: Cookies<'_>, _user: LenientForm<FormNewUser>) -> Json<String> {
    use crate::models::{NewAddress, NewPhone, NewSysUser};
    use crate::schema::{address, phone, sysuser};

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&_user.user_pass);

    let birth_split: Vec<&str> = _user.user_birthdate.split("-").collect();

    let connection = establish_connection();
    let mut user_inserted_id: i32 = 0_i32;
    /* This value is used as an index of the vector of error messages 
    * Default value is invalid_date_error 
    * In this case, the value default prevent a repetition of the attribution of this in the user_birthdate validations */
    let mut error_index: String = "invalid_date_error".to_string(); 

    /* Begin transaction */
    match connection.build_transaction()
        .read_write() 
        .run::<_, diesel::result::Error, _>(|| {
            /* Trying to insert the user data */
            match diesel::insert_into(sysuser::table)
                .values(NewSysUser {
                    user_name: _user.user_name.to_string(),
                    user_email: _user.user_email.to_string(),
                    user_password: hasher.result_str(),
                    /* from_ymd_opt allows to work with the Option result and is very util on error parsing */
                    user_birthdate: match NaiveDate::from_ymd_opt(
                        /* trying to parse birth_split[0] as i32 */
                        match birth_split[0].parse::<i32>() {
                            Ok(year) => { year }
                            Err(_) => { return Err(diesel::result::Error::NotFound) }
                        },
                        /* trying to parse birth_split[0] as u32 */
                        match birth_split[1].parse::<u32>() {
                            Ok(month) => { month }
                            Err(_) => { return Err(diesel::result::Error::NotFound) }
                        },
                        /* trying to parse birth_split[0] as u32 */
                        match birth_split[2].parse::<u32>() {
                            Ok(day) => { day }
                            Err(_) => { return Err(diesel::result::Error::NotFound) }
                        },
                    ) {
                        /* BirthDate is OK */
                        Some(birthdate) => { birthdate }
                        /* In this case, the date has a valid format but not is a valid date, for example, 30/02/2020 */
                        None => { return Err(diesel::result::Error::NotFound); }
                    },
                    user_genre: _user.user_genre.to_string(),
                    user_alias: Some(_user.user_alias.to_string()),
                    user_newsletter: true,
                    user_creation: Utc::now().naive_utc(),
                    user_lasttimeonline: Some(Utc::now().naive_utc()),
                    user_balance: 0.00,
                    user_bonus: 0.00,
                    user_type_id: 1,
                    user_status: Some(true),
                    user_uni: Some(_user.user_uni.to_string()),
                    user_fb_id: Some("".to_string()),
                })
                .returning(sysuser::user_id)
                .get_result(&connection) {
                /* if it's OK, set the user_inserted_id to id */
                Ok(id) => { user_inserted_id = id }
                /* If has an error, set the error_index to an adequated value  and return de adequated Err type */
                Err(e) => { 
                    error_index = "user_error".to_string();                    
                    return Err(e);
                }
            }
            /* Inserting the address data */
            match diesel::insert_into(address::table)
                .values(NewAddress {
                    address_number: _user.user_addr_num.to_string(),
                    address_street: _user.user_street.to_string(),
                    address_city: _user.user_city.to_string(),
                    address_state: "Portugal".to_string(),
                    address_country: "Portugal".to_string(),
                    address_postalcode: _user.user_zip_code.to_string(),
                    user_id: user_inserted_id,
                })
                .execute(&connection) {
                /* if it's OK, set the address_inserted_id to id -  */    
                Ok(_) => { }
                /* If has an error, set the error_index to an adequated value  and return de adequated Err type */
                Err(e) => { 
                    error_index = "address_error".to_string();
                    return Err(e);
                }
            }
            /* Inserting the phone data */
            match diesel::insert_into(phone::table)
                .values(NewPhone {
                    phone_number: _user.user_phone.to_string(),
                    user_id: user_inserted_id,
                    phone_type_id: 1,
                })
                .execute(&connection) {
                Ok(_) => { }
                /* If has an error, set the error_index to an adequated value  and return de adequated Err type */
                Err(e) => { 
                    error_index = "phone_error".to_string();                
                    return Err(e);
                }
            }
            /* If has arrived here, it's working fine and all data has inserted
            *  COMMIT and ROLLBACK is called implicitly
            */
            Ok(())
    }) {
        Ok(_) => { 
            cookies.add_private(Cookie::new("user_id", user_inserted_id.to_string()));
            /* Signaling to the Front-end  that the process was successful */
            Json("success".to_string())
        }
        Err(e) => { 
            /* Logging the error */
            crate::controller::logs::panics::manual_log(
                String::from("Usr-Register_Error"), 
                format!("User data => {:?}", _user), 
                e.to_string()
            );
            /* Returning adequated error to frontend for it display the adequated message */
            Json(error_index)
        }
    }
  
    /* Inserting user 
    let user_inserted_id: i32 = diesel::insert_into(sysuser::table)
        .values(NewSysUser {
            user_name: _user.user_name.to_string(),
            user_email: _user.user_email.to_string(),
            user_password: hasher.result_str(),
            user_birthdate: NaiveDate::from_ymd(
                birth_split[0].parse().unwrap(),
                birth_split[1].parse().unwrap(),
                birth_split[2].parse().unwrap(),
            ),
            user_genre: _user.user_genre.to_string(),
            user_alias: Some(_user.user_alias.to_string()),
            user_newsletter: true,
            user_creation: Utc::now().naive_utc(),
            user_lasttimeonline: Some(Utc::now().naive_utc()),
            user_balance: 0.00,
            user_bonus: 0.00,
            user_type_id: 1,
            user_status: Some(true),
            user_uni: Some(_user.user_uni.to_string()),
            user_fb_id: Some("".to_string()),
        })
        .returning(sysuser::user_id)
        .get_result(&connection)
        .unwrap();

    
    diesel::insert_into(address::table)
        .values(NewAddress {
            address_number: _user.user_addr_num.to_string(),
            address_street: _user.user_street.to_string(),
            address_city: _user.user_city.to_string(),
            address_state: "Portugal".to_string(),
            address_country: "Portugal".to_string(),
            address_postalcode: _user.user_zip_code.to_string(),
            user_id: user_inserted_id,
        })
        .execute(&connection)
        .unwrap();

    
    diesel::insert_into(phone::table)
        .values(NewPhone {
            phone_number: _user.user_phone.to_string(),
            user_id: user_inserted_id,
            phone_type_id: 1,
        })
        .execute(&connection)
        .unwrap();

    cookies.add_private(Cookie::new("user_id", user_inserted_id.to_string()));

    Redirect::to("/my-acc")*/
}
