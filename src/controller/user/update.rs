/*
888     888 8888888b.  8888888b.        d8888 88888888888 8888888 888b    888  .d8888b.
888     888 888   Y88b 888  "Y88b      d88888     888       888   8888b   888 d88P  Y88b
888     888 888    888 888    888     d88P888     888       888   88888b  888 888    888
888     888 888   d88P 888    888    d88P 888     888       888   888Y88b 888 888
888     888 8888888P"  888    888   d88P  888     888       888   888 Y88b888 888  88888
888     888 888        888    888  d88P   888     888       888   888  Y88888 888    888
Y88b. .d88P 888        888  .d88P d8888888888     888       888   888   Y8888 Y88b  d88P
 "Y88888P"  888        8888888P" d88P     888     888     8888888 888    Y888  "Y8888P88
*/

//Import estabilish connection from main
use crate::establish_connection;

//Lenient form imports
use chrono::{NaiveDate};
use diesel::prelude::*;
use rocket::request::LenientForm;

//Pass hashing
use crypto::digest::Digest;
use crypto::sha2::Sha512;

/* Redirect */
use rocket::response::Redirect;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

/*
    Here user _ have an underline for easy identifying thats
    a temporary and a can fail post (when not matching struct)
*/
pub fn check_pass(user_id: i32, _pass_hash: String) -> bool {
    use crate::schema::*;
    let results = sysuser::table
        .select(sysuser::user_password)
        .filter(sysuser::user_id.eq(user_id))
        .load::<String>(&establish_connection())
        .expect(
            "Some Error occured while parsing password hash reference. It was Registered in logs.",
        );

    if results[0] == _pass_hash {
        true
    } else {
        false
    }
}

//Form struct (must match to what is beeing posted) - Do not need to match the struct of the
// NewUser (Cuz form is diff of a Table Structure, both have some and dont some fields. )
#[derive(FromForm, Debug)]
pub struct FormNewUser {
    pub user_name: String,
    pub user_email: String,
    pub user_birthdate: String,
    pub user_genre: String,
    pub user_alias: String,
    pub user_pass: String,
}

use crate::User;
#[post("/self-update", data = "<_user>")]
pub fn update_user(user: User, _user: LenientForm<FormNewUser>) -> Redirect {
    use crate::schema::sysuser;

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&_user.user_pass);

    let birth_split: Vec<&str> = _user.user_birthdate.split("-").collect();

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user.user_id as i32)))
        .set((
            sysuser::user_name.eq(_user.user_name.to_string()),
            sysuser::user_birthdate.eq(NaiveDate::from_ymd(
                birth_split[0].parse().unwrap(),
                birth_split[1].parse().unwrap(),
                birth_split[2].parse().unwrap(),
            )),
            sysuser::user_alias.eq(_user.user_alias.to_string()),
            sysuser::user_password.eq(
                if check_pass(user.user_id as i32, _user.user_pass.clone()) {
                    _user.user_pass.to_string()
                } else {
                    hasher.result_str()
                },
            ),
        ))
        .execute(&crate::establish_connection())
        .expect("Shit.");

    Redirect::to("/my-acc")
}

use crate::AdminUser;

#[get("/new-pass/<user_id>")]
pub fn generate_new_pass(_administrative: AdminUser, user_id: i32) -> String {
    use crate::schema::sysuser;

    let mut rng = thread_rng();

    /* New Password Content */
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(10)
        .collect();

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&chars);

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id)))
        .set(sysuser::user_password.eq(hasher.result_str()))
        .execute(&crate::establish_connection())
        .expect("Wasn't able to set a new pass for this guy");

    let mail = sysuser::table
        .select(sysuser::user_email)
        .filter(sysuser::user_id.eq(user_id))
        .load::<String>(&crate::establish_connection())
        .expect("Missed ref");

    /* Post new pass */
    crate::controller::mail::send::pass_mail(chars.clone(), mail[0].clone().to_string());

    chars 
}

#[get("/user/new-pass-by-mail/<mail>")]
pub fn generate_new_pass_by_mail(mail: String) {
    use crate::schema::sysuser;

    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(10)
        .collect();

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&chars);

    diesel::update(sysuser::table.filter(sysuser::user_email.eq(mail.clone())))
        .set(sysuser::user_password.eq(hasher.result_str()))
        .execute(&crate::establish_connection())
        .expect("Wasn't able to set a new pass for this guy");

    /* Post new pass */
    crate::controller::mail::send::pass_mail(chars, mail.to_owned());
}
