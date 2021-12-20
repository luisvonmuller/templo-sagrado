/* When I coded this only me and god knew what I was doing, now only god knows */
use crate::AdminUser;
use crate::User;
use chrono::Utc;
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

#[get("/register-new-voice-chat-transaction/<chat_id>/<clerk_id>")]
pub fn register_voice_chat_transaction(
    user: User,
    chat_id: i32,
    clerk_id: i32,
) -> Result<Json<(i32, f64)>, Json<bool>> {
    use crate::models::NewVoiceChatTransaction;
    use crate::schema::{sysuser, voice_chat_transaction};

    /* Check if chat exists and also belongs to this clerk and if its running */
    if check_voice_chat(chat_id, user.user_id as i32, clerk_id) {
        let voice_chat_transaction_id: Vec<i32> = diesel::insert_into(voice_chat_transaction::table)
            .values(NewVoiceChatTransaction {
                voice_chat_transaction_value: (crate::get_values()).1,
                voice_chat_transaction_value_pay_off: ((crate::get_values()).1
                    * crate::controller::home::comission_rate(clerk_id)),
                voice_chat_transaction_chat_id: chat_id,
                voice_chat_transaction_paid_balance: None,
                voice_chat_transaction_paid_bonus: None,
                voice_chat_transaction_client_signature: None,
                voice_chat_transaction_clerk_signature: None,
                voice_chat_transaction_client_id: user.user_id as i32,
                voice_chat_transaction_clerk_id: clerk_id,
                voice_chat_transaction_creation: Utc::now().naive_utc(),
                voice_chat_transaction_update_client_signature: None,
                voice_chat_transaction_update_clerk_signature: None,
            })
            .returning(voice_chat_transaction::voice_chat_transaction_id)
            .get_results(&crate::establish_connection())
            .unwrap();

        let user_balance: Vec<f64> = sysuser::table
            .select(sysuser::user_balance)
            .filter(sysuser::user_id.eq(user.user_id))
            .load::<f64>(&crate::establish_connection())
            .expect("Entrou em parafuso");

        if voice_chat_transaction_id.len() > 0 {
            /*
                Will return the OK as a tuple, on postion 1 standing for the new transaction id and the
                second position beeing the balance disponible amount.
            */
            Ok(Json((voice_chat_transaction_id[0], user_balance[0])))
        } else {
            Err(Json(false))
        }
    } else {
        Err(Json(false))
    }
}

#[get("/client-sign-voice-chat-transaction/<voice_chat_transaction_id>")]
pub fn client_sign_voice_chat_transaction(
    user: User,
    voice_chat_transaction_id: i32,
) -> Result<Json<bool>, Json<bool>> {
    use crate::schema::{sysuser, voice_chat_transaction};

    /*
        Stands for both user_balance and user_bonus (amount given by system admin)
    */
    let (user_balance, user_bonus): (f64, f64) = sysuser::table
        .select((sysuser::user_balance, sysuser::user_bonus))
        .filter(sysuser::user_id.eq(user.user_id))
        .load::<(f64, f64)>(&crate::establish_connection())
        .expect("No user found")[0];

    let transaction_value: f64 = voice_chat_transaction::table
        .select(voice_chat_transaction::voice_chat_transaction_value)
        .filter(voice_chat_transaction::voice_chat_transaction_id.eq(voice_chat_transaction_id))
        .load::<f64>(&crate::establish_connection())
        .expect("No values found so far.")[0];

    if voice_payout(
        voice_chat_transaction_id,
        user.user_id,
        user_balance,
        user_bonus,
        transaction_value,
    ) {
        Ok(Json(true))
    } else {
        Err(Json(false))
    }
}


pub fn voice_payout(
    transaction_id: i32,
    client_id: i32,
    user_balance: f64,
    user_bonus: f64,
    amount: f64,
) -> bool {
    use crate::schema::{sysuser, voice_chat_transaction};
    use diesel::prelude::*;

    /* precheck of amount for performance improvements
    - First we will need to check out if it actually have the amount to complete the whole transaction, blocking any attemp without actually being able to pay all the debts
     */
    if (user_balance + user_bonus) >= amount {
        /* The var contents is the rest of the bonus_balance or the amount that ramnant not paid */
        let diff: f64 = user_bonus - amount;
        /* Diff negative generate a credit to client balance */
        if diff >= 0.0 {
            /* Paid everything with account's bonus */
            diesel::update(
                voice_chat_transaction::table
                    .filter(voice_chat_transaction::voice_chat_transaction_id.eq(transaction_id)),
            )
            .set((
                voice_chat_transaction::voice_chat_transaction_paid_bonus.eq(amount),
                voice_chat_transaction::voice_chat_transaction_client_signature
                    .eq(super::sign_hash()),
                voice_chat_transaction::voice_chat_transaction_update_client_signature
                    .eq(Utc::now().naive_utc()),
            ))
            .execute(&crate::establish_connection())
            .expect("We cannot update this, dude.");

            /* If we penetrated into this arm, the diff stands for the bonus remnant amount */
            diesel::update(sysuser::table.filter(sysuser::user_id.eq(client_id)))
                .set(sysuser::user_bonus.eq(diff))
                .execute(&crate::establish_connection())
                .expect("We cannot update this, dude.");
        } else {
            /* When entering this stuff here, we can assume for sure that theres no way that user bonus balance is over 0 */
            diesel::update(sysuser::table.filter(sysuser::user_id.eq(client_id)))
                .set(sysuser::user_bonus.eq(0.0))
                .execute(&crate::establish_connection())
                .expect("We cannot update this, dude.");

            /* Didn't paid anything */
            if (amount - diff) == amount {
                voice_payout_from_balance(transaction_id, client_id, amount);
                println!("true {:?}", amount);

            /* Did paid something */
            } else {
                /*
                Do insert the amount paid from bonus into the assignment field on the voice_transaction table, then
                with the rest of the value, behave paying from the chat.
                **Insert into database**
                */
                let value_paid_from_bonus = amount + diff;

                /* Lets register the amount that we did have paid from user_bonus */
                diesel::update(
                    voice_chat_transaction::table
                        .filter(voice_chat_transaction::voice_chat_transaction_id.eq(transaction_id)),
                )
                .set((
                    voice_chat_transaction::voice_chat_transaction_paid_bonus
                        .eq(value_paid_from_bonus),
                    voice_chat_transaction::voice_chat_transaction_client_signature
                        .eq(super::sign_hash()),
                    voice_chat_transaction::voice_chat_transaction_update_client_signature
                        .eq(Utc::now().naive_utc()),
                ))
                .execute(&crate::establish_connection())
                .expect("We cannot update this, dude.");

                voice_payout_from_balance(
                    transaction_id,
                    client_id,
                    amount - (value_paid_from_bonus),
                );
            }
        }
        true
    } else {
        /* User don't have enough credits, so no transaction will be done */
        false
    }
}

/*
    If user can't afford the transaction only with his bonus, he will pay from his balance * if theres some
*/
pub fn voice_payout_from_balance(transaction_id: i32, client_id: i32, to_pay: f64) -> () {
    use crate::schema::{sysuser, voice_chat_transaction};
    use diesel::prelude::*;

    /* Lets register the amount that we did have paid from user_bonus */
    diesel::update(
        voice_chat_transaction::table
            .filter(voice_chat_transaction::voice_chat_transaction_id.eq(transaction_id)),
    )
    .set((
        voice_chat_transaction::voice_chat_transaction_paid_balance.eq(to_pay),
        voice_chat_transaction::voice_chat_transaction_client_signature.eq(super::sign_hash()),
        voice_chat_transaction::voice_chat_transaction_update_client_signature
            .eq(Utc::now().naive_utc()),
    ))
    .execute(&crate::establish_connection())
    .expect("We cannot update this, dude.");

    /* If we penetrated into this arm, the diff stands for the bonus remnant amount */
    diesel::update(sysuser::table.filter(sysuser::user_id.eq(client_id)))
        .set(sysuser::user_balance.eq(sysuser::user_balance - to_pay ))
        .execute(&crate::establish_connection())
        .expect("We cannot update this, dude.");
}

#[get("/clerk-sign-voice-chat-transaction/<voice_chat_transaction_id>")]
pub fn clerk_sign_voice_chat_transaction(
    user: User,
    voice_chat_transaction_id: i32,
) -> Result<Json<bool>, Json<bool>> {
    use crate::schema::voice_chat_transaction;

    if verify_client_signature(voice_chat_transaction_id) {
        let payoff_val: Vec<f64> =
            diesel::update(voice_chat_transaction::table.filter(
                voice_chat_transaction::voice_chat_transaction_id.eq(voice_chat_transaction_id),
            ))
            .set((
                voice_chat_transaction::voice_chat_transaction_clerk_signature
                    .eq(Some(super::sign_hash().to_string())),
                voice_chat_transaction::voice_chat_transaction_update_clerk_signature
                    .eq(Some(Utc::now().naive_utc())),
            ))
            .returning(voice_chat_transaction::voice_chat_transaction_value_pay_off)
            .get_results(&crate::establish_connection())
            .unwrap();

        if payoff_val.len() > 0 {
            crate::controller::payments::give_cesar_what_belongs_to_cesar(
                user.user_id as i32,
                payoff_val[0],
            );
            
            /* Downs a minute from global total */
            crate::voice_minutes_take_off();
            
            Ok(Json(true))
        } else {
            Err(Json(false))
        }
    } else {
        Err(Json(false))
    }
}

fn verify_client_signature(_voice_chat_transaction_id: i32) -> bool {
    use crate::schema::voice_chat_transaction;

    diesel::dsl::select(diesel::dsl::exists(
        voice_chat_transaction::table
            .select(voice_chat_transaction::voice_chat_transaction_id)
            .filter(voice_chat_transaction::voice_chat_transaction_client_signature.is_not_null()),
    ))
    .get_results(&crate::establish_connection())
    .unwrap()[0]
}

fn check_voice_chat(call_id: i32, user_id: i32, clerk_id: i32) -> bool {
    use crate::schema::call;

    diesel::dsl::select(diesel::dsl::exists(
        call::table
            .select(call::call_id)
            .filter(call::call_id.eq(call_id))
            .filter(call::user_id.eq(user_id))
            .filter(call::clerk_id.eq(clerk_id)),
    ))
    .get_results(&crate::establish_connection())
    .unwrap()[0]
}

#[get("/clerk-voice-chat-amount-owned/<chat_id>")]
pub fn clerk_voice_chat_amount_owned(
    user: User,
    chat_id: i32,
) -> Result<Json<(f64, f64)>, Json<bool>> {
    use crate::schema::{sysuser, voice_chat_transaction};
    use diesel::dsl::sum;
    use diesel::prelude::*;

    let amount: Vec<Option<f64>> = voice_chat_transaction::table
        .select(sum(
            voice_chat_transaction::voice_chat_transaction_value_pay_off,
        ))
        .filter(voice_chat_transaction::voice_chat_transaction_chat_id.eq(chat_id))
        .filter(voice_chat_transaction::voice_chat_transaction_clerk_id.eq(user.user_id as i32))
        .load::<Option<f64>>(&crate::establish_connection())
        .expect("We couldn't retrieve clerk amount owned by this service");

    let balance: Vec<f64> = sysuser::table.select(sysuser::user_balance).filter(sysuser::user_id.eq(user.user_id as i32)).load::<f64>(&crate::establish_connection()).expect("We couldn't retrieve clerk balance while getting stuff to display on chat_clerk.html.hbs");

    if amount.len() > 0 && balance.len() > 0 {
        let amount_value: f64 = match amount[0] {
            Some(val) => val,
            None => 0.0,
        };
        /* On front I should verify with a null coalescing operator */
        Ok(Json((amount_value, balance[0])))
    } else {
        Err(Json(false))
    }
}

/*
       d8888 8888888b.  888b     d888 8888888 888b    888      8888888 888b     d888 8888888b.  888      8888888888 888b     d888 8888888888 888b    888 88888888888     d8888 88888888888 8888888 .d88888b.  888b    888  .d8888b.
      d88888 888  "Y88b 8888b   d8888   888   8888b   888        888   8888b   d8888 888   Y88b 888      888        8888b   d8888 888        8888b   888     888        d88888     888       888  d88P" "Y88b 8888b   888 d88P  Y88b
     d88P888 888    888 88888b.d88888   888   88888b  888        888   88888b.d88888 888    888 888      888        88888b.d88888 888        88888b  888     888       d88P888     888       888  888     888 88888b  888 Y88b.
    d88P 888 888    888 888Y88888P888   888   888Y88b 888        888   888Y88888P888 888   d88P 888      8888888    888Y88888P888 8888888    888Y88b 888     888      d88P 888     888       888  888     888 888Y88b 888  "Y888b.
   d88P  888 888    888 888 Y888P 888   888   888 Y88b888        888   888 Y888P 888 8888888P"  888      888        888 Y888P 888 888        888 Y88b888     888     d88P  888     888       888  888     888 888 Y88b888     "Y88b.
  d88P   888 888    888 888  Y8P  888   888   888  Y88888        888   888  Y8P  888 888        888      888        888  Y8P  888 888        888  Y88888     888    d88P   888     888       888  888     888 888  Y88888       "888
 d8888888888 888  .d88P 888   "   888   888   888   Y8888        888   888   "   888 888        888      888        888   "   888 888        888   Y8888     888   d8888888888     888       888  Y88b. .d88P 888   Y8888 Y88b  d88P
d88P     888 8888888P"  888       888 8888888 888    Y888      8888888 888       888 888        88888888 8888888888 888       888 8888888888 888    Y888     888  d88P     888     888     8888888 "Y88888P"  888    Y888  "Y8888P"


# Great steps to follow:

 -> For paying back:
        1. Count all given amount
        2. Count all HOLD amount
        3. Take out from the Clerk the amount given
        4. Recred the amount payed

        Needs admin auths (as a receiver of AdminUser)
        Needs the chat identifier (as an i32)
*/

/* Macros and other stuffs */
use crate::models::Sum;
use diesel::*;

#[get("/voice-chat-payback/<voice_chat_id>")]
pub fn voice_pay_back(_administrative: AdminUser, voice_chat_id: i32) {
    use crate::schema::{sysuser, voice_chat_transaction};

    let (paid, _given, _hold, rate) = (
        voice_paid_amount(voice_chat_id),
        voice_given_amount(voice_chat_id),
        hold_amount(voice_chat_id),
        crate::controller::home::comission_rate(super::voice_which_clerk(voice_chat_id)),
    );

    match paid.sum {
        Some(paid_val) => {
            /* Take out the value paid for the clerk */
            diesel::update(
                sysuser::table.filter(sysuser::user_id.eq(super::voice_which_clerk(voice_chat_id))),
            )
            .set(sysuser::user_balance.eq(sysuser::user_balance - (paid_val * rate)))
            .execute(&crate::establish_connection())
            .expect("We cannot update this, dude.");

            /* Give ceaser whats belongs to ceaser */
            diesel::update(
                sysuser::table.filter(sysuser::user_id.eq(super::voice_which_client(voice_chat_id))),
            )
            .set(sysuser::user_balance.eq(sysuser::user_balance + paid_val))
            .execute(&crate::establish_connection())
            .expect("We cannot update this, dude.");
        }
        None => {
            //Nothing to do lol.
        }
    }

    /* Resign (Clean On Hold Value and paid values) */
    diesel::update(
        voice_chat_transaction::table
            .filter(voice_chat_transaction::voice_chat_transaction_chat_id.eq(voice_chat_id)),
    )
    .set((
        voice_chat_transaction::voice_chat_transaction_clerk_signature.eq::<Option<String>>(None),
        voice_chat_transaction::voice_chat_transaction_client_signature.eq::<Option<String>>(None),
    ))
    .execute(&crate::establish_connection())
    .expect("Fon");
}

/*
# Great steps to follow:

 -> For paying back:
        1. Check for non-signed client transactions and sign'em takin out the value from
        the client
        2. Count all HOLD amount values not paid to the client
        3. Give this amount to the clerk

        Needs admin auths (as a receiver of AdminUser)
        Needs the chat identifier (as an i32)

*/
#[get("/voice-chat-pay-clerk/<voice_chat_id>")]
pub fn pay_clerk(_administrative: AdminUser, voice_chat_id: i32) {
    use crate::schema::{sysuser, voice_chat_transaction};

    let (hold, rate) = (
        hold_amount(voice_chat_id),
        crate::controller::home::comission_rate(super::voice_which_clerk(voice_chat_id)),
    );

    match hold.sum {
        Some(holding_val) => {
            /* Take out the value paid for the clerk */
            diesel::update(
                sysuser::table.filter(sysuser::user_id.eq(super::voice_which_clerk(voice_chat_id))),
            )
            .set(sysuser::user_balance.eq(sysuser::user_balance + (holding_val * rate)))
            .execute(&crate::establish_connection())
            .expect("We cannot update this, dude.");
        }
        None => {
            //Nothing to do lol
        }
    }

    /* Resign all as it was signed by the system */
    diesel::update(
        voice_chat_transaction::table
            .filter(voice_chat_transaction::voice_chat_transaction_chat_id.eq(voice_chat_id)),
    )
    .set(voice_chat_transaction::voice_chat_transaction_clerk_signature.eq(super::sys_sign()))
    .execute(&crate::establish_connection())
    .expect("We cannot sign as sys, dude.");
}

fn voice_paid_amount(voice_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(voice_chat_transaction_value) FROM voice_chat_transaction WHERE voice_chat_transaction_chat_id={} AND voice_chat_transaction_client_signature IS NOT NULL",
       voice_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn voice_given_amount(voice_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(voice_chat_transaction_value) FROM voice_chat_transaction WHERE voice_chat_transaction_chat_id={} AND voice_chat_transaction_clerk_signature IS NOT NULL",
       voice_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn hold_amount(voice_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(voice_chat_transaction_value) FROM voice_chat_transaction WHERE voice_chat_transaction_chat_id={} AND voice_chat_transaction_clerk_signature IS NULL AND voice_chat_transaction_client_signature IS NOT NULL",
       voice_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn voice_processed_value(voice_chat_id: i32) -> Sum {
    let count: Sum = sql_query(format!(
        "SELECT sum(voice_chat_transaction_value) FROM voice_chat_transaction WHERE voice_chat_transaction_chat_id={}",
       voice_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

/* ADMIN GETTER INTERFACES  */

/* Admin Front-end interfaces */
#[get("/voice_chat_paid/<id>")]
pub fn get_paid(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(voice_paid_amount(id))
}

#[get("/voice_chat_given/<id>")]
pub fn get_given(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(voice_given_amount(id))
}

#[get("/voice_chat_hold/<id>")]
pub fn get_hold(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(hold_amount(id))
}

#[get("/voice_processed_value/<id>")]
pub fn get_procesed(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(voice_processed_value(id))
}

#[get("/voice-all-time-paid")]
pub fn all_time_paid(_admin: AdminUser) -> Json<Sum> {
    let count: Sum = sql_query(
        "SELECT sum(voice_chat_transaction_value) FROM voice_chat_transaction WHERE voice_chat_transaction_client_signature IS NOT NULL"
    )
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}

#[get("/voice-all-time-given")]
pub fn all_time_given(_admin: AdminUser) -> Json<Sum> {
    let count: Sum = sql_query(
        "SELECT sum(voice_chat_transaction_value_pay_off) FROM voice_chat_transaction WHERE voice_chat_transaction_clerk_signature IS NOT NULL"
       )
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}

#[get("/voice-all-time-processed")]
pub fn all_time_processed(_admin: AdminUser) -> Json<Sum> {
    let count: Sum =
        sql_query("SELECT sum(voice_chat_transaction_value) FROM voice_chat_transaction")
            .load::<Sum>(&crate::establish_connection())
            .expect("Query failed")
            .pop()
            .expect("No rows");

    Json(count)
}

#[get("/voice-all-time-paid-from-cash")]
pub fn all_time_paid_from_cash(_admin: AdminUser) -> Json<Sum> {
    let count: Sum =
        sql_query("SELECT sum(voice_chat_transaction_paid_balance) FROM voice_chat_transaction")
            .load::<Sum>(&crate::establish_connection())
            .expect("Query failed")
            .pop()
            .expect("No rows");

    Json(count)
}

#[get("/voice-all-time-paid-from-bonus")]
pub fn all_time_paid_from_bonus(_admin: AdminUser) -> Json<Sum> {
    let count: Sum =
        sql_query("SELECT sum(voice_chat_transaction_paid_bonus) FROM voice_chat_transaction")
            .load::<Sum>(&crate::establish_connection())
            .expect("Query failed")
            .pop()
            .expect("No rows");

    Json(count)
}
