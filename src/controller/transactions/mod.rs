/* Move all to a module called text_chat_transactions */
pub mod text_chat;
pub mod voice_chat;

pub fn sign_hash() -> String {
    /* Random Alphanumeric digits as a simple hash String */
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::iter;

    let mut rng = thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(64)
        .collect()
}

/* On system signs this will be calld */
pub fn sys_sign() -> String {
    String::from("SignedAsSystem")
}

/* I should implement a balance check but nah, joke ill */
pub fn client_balance_is_worth_for_a_minute_of_text_chat(
    user_id: i32,
    transaction_id: i32,
) -> bool {
    use crate::schema::{sysuser, text_chat_transaction};
    use diesel::prelude::*;

    /* Get users balance */
    let user_balance: Vec<f64> = sysuser::table
        .select(sysuser::user_balance)
        .filter(sysuser::user_id.eq(user_id))
        .load::<f64>(&crate::establish_connection())
        .expect("No user found");

    /* Get debit value */
    let debit: Vec<f64> = text_chat_transaction::table
        .select(text_chat_transaction::text_chat_transaction_value)
        .filter(text_chat_transaction::text_chat_transaction_id.eq(transaction_id))
        .load::<f64>(&crate::establish_connection())
        .expect("Something wrong occured while parsing transaction debit value");

    /* Since it return a Vec<f64> we need to first match it */
    if user_balance.len() > 0 && debit.len() > 0 {
        if user_balance[0] >= debit[0] {
            true
        } else {
            false
        }
    } else {
        false
    }
}

/* 
    Stands for the verification of client credits for voice transactions
*/
pub fn client_balance_is_worth_for_a_minute_of_voice_chat(
    user_id: i32,
    transaction_id: i32,
) -> bool {
    use crate::schema::{sysuser, voice_chat_transaction};
    use diesel::prelude::*;

    /* Get users balance */
    let user_balance: Vec<f64> = sysuser::table
        .select(sysuser::user_balance)
        .filter(sysuser::user_id.eq(user_id))
        .load::<f64>(&crate::establish_connection())
        .expect("No user found");

    /* Get debit value */
    let debit: Vec<f64> = voice_chat_transaction::table
        .select(voice_chat_transaction::voice_chat_transaction_value)
        .filter(voice_chat_transaction::voice_chat_transaction_id.eq(transaction_id))
        .load::<f64>(&crate::establish_connection())
        .expect("Something wrong occured while parsing transaction debit value");

    /* Since it return a Vec<f64> we need to first match it */
    if user_balance.len() > 0 && debit.len() > 0 {
        if user_balance[0] >= debit[0] {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn text_which_clerk(chat_id: i32) -> i32 {
    use crate::schema::text_chat_transaction;
    use diesel::prelude::*;

    text_chat_transaction::table
        .select(text_chat_transaction::text_chat_transaction_clerk_id)
        .filter(text_chat_transaction::text_chat_transaction_chat_id.eq(chat_id))
        .limit(1)
        .load::<i32>(&crate::establish_connection())
        .expect("whoops")[0]
}

pub fn voice_which_clerk(chat_id: i32) -> i32 {
    use crate::schema::voice_chat_transaction;
    use diesel::prelude::*;

    voice_chat_transaction::table
        .select(voice_chat_transaction::voice_chat_transaction_clerk_id)
        .filter(voice_chat_transaction::voice_chat_transaction_chat_id.eq(chat_id))
        .limit(1)
        .load::<i32>(&crate::establish_connection())
        .expect("whoops")[0]
}


pub fn text_which_client(chat_id: i32) -> i32 {
    use crate::schema::text_chat_transaction;
    use diesel::prelude::*;

    text_chat_transaction::table
        .select(text_chat_transaction::text_chat_transaction_client_id)
        .filter(text_chat_transaction::text_chat_transaction_chat_id.eq(chat_id))
        .limit(1)
        .load::<i32>(&crate::establish_connection())
        .expect("whoops")[0]
}

pub fn voice_which_client(chat_id: i32) -> i32 {
    use crate::schema::voice_chat_transaction;
    use diesel::prelude::*;

    voice_chat_transaction::table
        .select(voice_chat_transaction::voice_chat_transaction_client_id)
        .filter(voice_chat_transaction::voice_chat_transaction_chat_id.eq(chat_id))
        .limit(1)
        .load::<i32>(&crate::establish_connection())
        .expect("whoops")[0]
}
