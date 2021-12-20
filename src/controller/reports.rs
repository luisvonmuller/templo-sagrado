/* diesel ORM base macros (must have) */
use diesel::prelude::*;

/* Our database structures needed as a return type */
use crate::models::{Address, ClerkBank, ClerkInfo, Phone, SysUser};

/* Template */
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/generate-payment-report")]
pub fn generate_payment_report() -> Template {
    /* Instanciate template holding hash map */
    let mut context = HashMap::new();

    /* Get data from database */
    let report_data = clerks_payments_data();

    /* Inserts into the context for parsing to the template */
    context.insert("clerks", report_data);

    /* Renders the template */
    Template::render("pages/reports/payment-report", &context)
}

pub fn clerks_payments_data() -> Vec<(SysUser, ClerkInfo, ClerkBank, Address, Phone)> {
    use crate::schema::{address, clerk_bank, clerk_info, phone, sysuser}; /* Macros of diesel */

    sysuser::table
        .inner_join(clerk_info::table)
        .inner_join(clerk_bank::table)
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            clerk_info::all_columns,
            clerk_bank::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .load::<(SysUser, ClerkInfo, ClerkBank, Address, Phone)>(&crate::establish_connection())
        .expect("Some shit happned while retrieving a single clerk!! <Panic at the Thread>")
}
