/* When I coded this only me and god knew what I was doing, now only god knows */
use crate::AdminUser;
use crate::User;
use chrono::Utc;
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

#[get("/register-new-text-chat-transaction/<chat_id>/<clerk_id>")]
pub fn register_text_chat_transaction(
    user: User,
    chat_id: i32,
    clerk_id: i32,
) -> Result<Json<(i32, f64)>, Json<bool>> {
    use crate::models::NewTextChatTransaction;
    use crate::schema::{sysuser, text_chat_transaction};

    /* Check if chat exists and also belongs to this clerk and if its running */
    if check_text_chat(chat_id, user.user_id as i32, clerk_id) {
        let text_chat_transaction_id: Vec<i32> = diesel::insert_into(text_chat_transaction::table)
            .values(NewTextChatTransaction {
                text_chat_transaction_value: (crate::get_values()).1,
                text_chat_transaction_value_pay_off: ((crate::get_values()).1
                    * crate::controller::home::comission_rate(clerk_id)),
                text_chat_transaction_paid_balance: None,
                text_chat_transaction_paid_bonus: None,
                text_chat_transaction_chat_id: chat_id,
                text_chat_transaction_client_signature: None,
                text_chat_transaction_clerk_signature: None,
                text_chat_transaction_client_id: user.user_id as i32,
                text_chat_transaction_clerk_id: clerk_id,
                text_chat_transaction_creation: Utc::now().naive_utc(),
                text_chat_transaction_update_client_signature: None,
                text_chat_transaction_update_clerk_signature: None,
            })
            .returning(text_chat_transaction::text_chat_transaction_id)
            .get_results(&crate::establish_connection())
            .unwrap();

        let user_balance: Vec<f64> = sysuser::table
            .select(sysuser::user_balance)
            .filter(sysuser::user_id.eq(user.user_id))
            .load::<f64>(&crate::establish_connection())
            .expect("Entrou em parafuso");

        if text_chat_transaction_id.len() > 0 {
            /*
                Will return the OK as a tuple, on postion 1 standing for the new transaction id and the
                second position beeing the balance disponible amount.
            */
            Ok(Json((text_chat_transaction_id[0], user_balance[0])))
        } else {
            Err(Json(false))
        }
    } else {
        Err(Json(false))
    }
}

#[get("/client-sign-text-chat-transaction/<text_chat_transaction_id>")]
pub fn client_sign_text_chat_transaction(
    user: User,
    text_chat_transaction_id: i32,
) -> Result<Json<bool>, Json<bool>> {
    use crate::schema::{sysuser, text_chat_transaction};

    /*
        Stands for both user_balance and user_bonus (amount given by system admin)
    */
    let (user_balance, user_bonus): (f64, f64) = sysuser::table
        .select((sysuser::user_balance, sysuser::user_bonus))
        .filter(sysuser::user_id.eq(user.user_id))
        .load::<(f64, f64)>(&crate::establish_connection())
        .expect("No user found")[0];

    let transaction_value: f64 = text_chat_transaction::table
        .select(text_chat_transaction::text_chat_transaction_value)
        .filter(text_chat_transaction::text_chat_transaction_id.eq(text_chat_transaction_id))
        .load::<f64>(&crate::establish_connection())
        .expect("No values found so far.")[0];

    if text_payout(
        text_chat_transaction_id,
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

#[get("/clerk-sign-text-chat-transaction/<text_chat_transaction_id>")]
pub fn clerk_sign_text_chat_transaction(
    user: User,
    text_chat_transaction_id: i32,
) -> Result<Json<bool>, Json<bool>> {
    use crate::schema::text_chat_transaction;

    if verify_client_signature(text_chat_transaction_id) {
        let payoff_val: Vec<f64> =
            diesel::update(text_chat_transaction::table.filter(
                text_chat_transaction::text_chat_transaction_id.eq(text_chat_transaction_id),
            ))
            .set((
                text_chat_transaction::text_chat_transaction_clerk_signature
                    .eq(Some(super::sign_hash().to_string())),
                text_chat_transaction::text_chat_transaction_update_clerk_signature
                    .eq(Some(Utc::now().naive_utc())),
            ))
            .returning(text_chat_transaction::text_chat_transaction_value_pay_off)
            .get_results(&crate::establish_connection())
            .unwrap();

        if payoff_val.len() > 0 {
            crate::controller::payments::give_cesar_what_belongs_to_cesar(
                user.user_id as i32,
                payoff_val[0],
            );

            Ok(Json(true))
        } else {
            Err(Json(false))
        }
    } else {
        Err(Json(false))
    }
}

fn verify_client_signature(_text_chat_transaction_id: i32) -> bool {
    use crate::schema::text_chat_transaction;

    diesel::dsl::select(diesel::dsl::exists(
        text_chat_transaction::table
            .select(text_chat_transaction::text_chat_transaction_id)
            .filter(text_chat_transaction::text_chat_transaction_client_signature.is_not_null()),
    ))
    .get_results(&crate::establish_connection())
    .unwrap()[0]
}

fn check_text_chat(chat_id: i32, user_id: i32, clerk_id: i32) -> bool {
    use crate::schema::chat;

    diesel::dsl::select(diesel::dsl::exists(
        chat::table
            .select(chat::chat_id)
            .filter(chat::chat_id.eq(chat_id))
            .filter(chat::client_id.eq(user_id))
            .filter(chat::clerk_id.eq(clerk_id)),
    ))
    .get_results(&crate::establish_connection())
    .unwrap()[0]
}

#[get("/clerk-text-chat-amount-owned/<chat_id>")]
pub fn clerk_text_chat_amount_owned(
    user: User,
    chat_id: i32,
) -> Result<Json<(f64, f64)>, Json<bool>> {
    use crate::schema::{sysuser, text_chat_transaction};
    use diesel::dsl::sum;
    use diesel::prelude::*;

    let amount: Vec<Option<f64>> = text_chat_transaction::table
        .select(sum(
            text_chat_transaction::text_chat_transaction_value_pay_off,
        ))
        .filter(text_chat_transaction::text_chat_transaction_chat_id.eq(chat_id))
        .filter(text_chat_transaction::text_chat_transaction_clerk_id.eq(user.user_id as i32))
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

pub fn text_payout(
    transaction_id: i32,
    client_id: i32,
    user_balance: f64,
    user_bonus: f64,
    amount: f64,
) -> bool {
    use crate::schema::{sysuser, text_chat_transaction};
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
                text_chat_transaction::table
                    .filter(text_chat_transaction::text_chat_transaction_id.eq(transaction_id)),
            )
            .set((
                text_chat_transaction::text_chat_transaction_paid_bonus.eq(amount),
                text_chat_transaction::text_chat_transaction_client_signature
                    .eq(super::sign_hash()),
                text_chat_transaction::text_chat_transaction_update_client_signature
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
                text_payout_from_balance(transaction_id, client_id, amount);
                println!("true {:?}", amount);

            /* Did paid something */
            } else {
                /*
                Do insert the amount paid from bonus into the assignment field on the text_transaction table, then
                with the rest of the value, behave paying from the chat.
                **Insert into database**
                */
                let value_paid_from_bonus = amount + diff;

                /* Lets register the amount that we did have paid from user_bonus */
                diesel::update(
                    text_chat_transaction::table
                        .filter(text_chat_transaction::text_chat_transaction_id.eq(transaction_id)),
                )
                .set((
                    text_chat_transaction::text_chat_transaction_paid_bonus
                        .eq(value_paid_from_bonus),
                    text_chat_transaction::text_chat_transaction_client_signature
                        .eq(super::sign_hash()),
                    text_chat_transaction::text_chat_transaction_update_client_signature
                        .eq(Utc::now().naive_utc()),
                ))
                .execute(&crate::establish_connection())
                .expect("We cannot update this, dude.");

                text_payout_from_balance(
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
pub fn text_payout_from_balance(transaction_id: i32, client_id: i32, to_pay: f64) -> () {
    use crate::schema::{sysuser, text_chat_transaction};
    use diesel::prelude::*;

    /* Lets register the amount that we did have paid from user_bonus */
    diesel::update(
        text_chat_transaction::table
            .filter(text_chat_transaction::text_chat_transaction_id.eq(transaction_id)),
    )
    .set((
        text_chat_transaction::text_chat_transaction_paid_balance.eq(to_pay),
        text_chat_transaction::text_chat_transaction_client_signature.eq(super::sign_hash()),
        text_chat_transaction::text_chat_transaction_update_client_signature
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

/*
# Administrative stuff
# Administrative stuff
# Administrative stuff
# Administrative stuff
# Administrative stuff
# Administrative stuff

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

#[get("/text-chat-payback/<text_chat_id>")]
pub fn text_pay_back(_administrative: AdminUser, text_chat_id: i32) {
    use crate::schema::{sysuser, text_chat_transaction};

    let (paid, _given, _hold, rate) = (
        text_paid_amount(text_chat_id),
        text_given_amount(text_chat_id),
        hold_amount(text_chat_id),
        crate::controller::home::comission_rate(super::text_which_clerk(text_chat_id)),
    );

    match paid.sum {
        Some(paid_val) => {
            /* Take out the value paid for the clerk */
            diesel::update(
                sysuser::table.filter(sysuser::user_id.eq(super::text_which_clerk(text_chat_id))),
            )
            .set(sysuser::user_balance.eq(sysuser::user_balance - (paid_val * rate)))
            .execute(&crate::establish_connection())
            .expect("We cannot update this, dude.");

            /* Give ceaser whats belongs to ceaser */
            diesel::update(
                sysuser::table.filter(sysuser::user_id.eq(super::text_which_client(text_chat_id))),
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
        text_chat_transaction::table
            .filter(text_chat_transaction::text_chat_transaction_chat_id.eq(text_chat_id)),
    )
    .set((
        text_chat_transaction::text_chat_transaction_clerk_signature.eq::<Option<String>>(None),
        text_chat_transaction::text_chat_transaction_client_signature.eq::<Option<String>>(None),
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
#[get("/text-chat-pay-clerk/<text_chat_id>")]
pub fn pay_clerk(_administrative: AdminUser, text_chat_id: i32) {
    use crate::schema::{sysuser, text_chat_transaction};

    let (hold, rate) = (
        hold_amount(text_chat_id),
        crate::controller::home::comission_rate(super::text_which_clerk(text_chat_id)),
    );

    match hold.sum {
        Some(holding_val) => {
            /* Take out the value paid for the clerk */
            diesel::update(
                sysuser::table.filter(sysuser::user_id.eq(super::text_which_clerk(text_chat_id))),
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
        text_chat_transaction::table
            .filter(text_chat_transaction::text_chat_transaction_chat_id.eq(text_chat_id)),
    )
    .set(text_chat_transaction::text_chat_transaction_clerk_signature.eq(super::sys_sign()))
    .execute(&crate::establish_connection())
    .expect("We cannot sign as sys, dude.");
}

fn text_paid_amount(text_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(text_chat_transaction_value) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={} AND text_chat_transaction_client_signature IS NOT NULL",
       text_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn text_paid_amount_bonus(text_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(text_chat_transaction_paid_bonus) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={} AND text_chat_transaction_client_signature IS NOT NULL",
       text_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn text_paid_amount_balance(text_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(text_chat_transaction_paid_balance) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={} AND text_chat_transaction_client_signature IS NOT NULL",
       text_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn text_given_amount(text_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(text_chat_transaction_value_pay_off) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={} AND text_chat_transaction_clerk_signature IS NOT NULL",
       text_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn hold_amount(text_chat_id: i32) -> Sum {
    /* Selects the total (SUM stmt) of transacted values */
    let count: Sum = sql_query(format!(
        "SELECT sum(text_chat_transaction_value) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={} AND text_chat_transaction_clerk_signature IS NULL AND text_chat_transaction_client_signature IS NOT NULL",
       text_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

fn text_processed_value(text_chat_id: i32) -> Sum {
    let count: Sum = sql_query(format!(
        "SELECT sum(text_chat_transaction_value) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={}",
       text_chat_id
    ))
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    count
}

/* ADMIN GETTER INTERFACES  */

/* Admin Front-end interfaces */
#[get("/text_chat_paid/<id>")]
pub fn get_paid(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(text_paid_amount(id))
}

#[get("/text_chat_bonus/<id>")]
pub fn get_paid_bonus(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(text_paid_amount_bonus(id))
}

#[get("/text_chat_balance/<id>")]
pub fn get_paid_balance(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(text_paid_amount_balance(id))
}

#[get("/text_chat_given/<id>")]
pub fn get_given(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(text_given_amount(id))
}

#[get("/text_chat_hold/<id>")]
pub fn get_hold(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(hold_amount(id))
}

#[get("/text_processed_value/<id>")]
pub fn get_procesed(_admin: AdminUser, id: i32) -> Json<Sum> {
    Json(text_processed_value(id))
}

#[get("/text-all-time-paid")]
pub fn all_time_paid(_admin: AdminUser) -> Json<Sum> {
    let count: Sum = sql_query(
        "SELECT sum(text_chat_transaction_value) FROM text_chat_transaction WHERE text_chat_transaction_client_signature IS NOT NULL"
    )
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}

#[get("/text-all-time-given")]
pub fn all_time_given(_admin: AdminUser) -> Json<Sum> {
    let count: Sum = sql_query(
        "SELECT sum(text_chat_transaction_value_pay_off) FROM text_chat_transaction WHERE text_chat_transaction_clerk_signature IS NOT NULL"
       )
    .load::<Sum>(&crate::establish_connection())
    .expect("Query failed")
    .pop()
    .expect("No rows");

    Json(count)
}

#[get("/text-all-time-processed")]
pub fn all_time_processed(_admin: AdminUser) -> Json<Sum> {
    let count: Sum =
        sql_query("SELECT sum(text_chat_transaction_value) FROM text_chat_transaction")
            .load::<Sum>(&crate::establish_connection())
            .expect("Query failed")
            .pop()
            .expect("No rows");

    Json(count)
}

#[get("/all-time-paid-from-cash")]
pub fn all_time_paid_from_cash(_admin: AdminUser) -> Json<Sum> {
    let count: Sum =
        sql_query("SELECT sum(text_chat_transaction_paid_balance) FROM text_chat_transaction")
            .load::<Sum>(&crate::establish_connection())
            .expect("Query failed")
            .pop()
            .expect("No rows");

    Json(count)
}

#[get("/all-time-paid-from-bonus")]
pub fn all_time_paid_from_bonus(_admin: AdminUser) -> Json<Sum> {
    let count: Sum =
        sql_query("SELECT sum(text_chat_transaction_paid_bonus) FROM text_chat_transaction")
            .load::<Sum>(&crate::establish_connection())
            .expect("Query failed")
            .pop()
            .expect("No rows");

    Json(count)
}
