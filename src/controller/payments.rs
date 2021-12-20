/* This function updates values from a completed transaction */
pub fn give_cesar_what_belongs_to_cesar(user_id: i32, value: f64) {
    use crate::schema::sysuser;
    use diesel::prelude::*;

    /* Just give over the sysuser balance */
    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id)))
        .set(sysuser::user_balance.eq(sysuser::user_balance + value))
        .execute(&crate::establish_connection())
        .unwrap();
}

pub fn give_cesar_what_belongs_to_cesar_with_product_id(user_id: i32, product_id: i32) {
    use crate::schema::{product, sysuser};
    use diesel::prelude::*;
    
    #[derive(Queryable)]
    struct ProductInfo {
        product_value: f64,
        product_bonus: f64,
    }

    /* Getting the product value and bonus */
    let results = product::table
        .select((product::product_value, product::product_bonus))
        .filter(product::product_id.eq(product_id))
        .load::<ProductInfo>(&crate::establish_connection())
        .expect("Some Error occured while getting product values. Registered in logs.");

    if results.len() > 0 {
        /* Just give over the sysuser balance and bonus */
        diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id)))
            .set((
                sysuser::user_balance.eq(sysuser::user_balance + results[0].product_value),
                sysuser::user_bonus.eq(sysuser::user_bonus + results[0].product_bonus),
            ))
            .execute(&crate::establish_connection())
            .unwrap();
    }
}
