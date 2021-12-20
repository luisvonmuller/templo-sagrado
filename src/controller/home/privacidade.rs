/* Home -> /Privacidade 
    [View structure] 
        [0] content
        [1] login data

*/

use super::*;

#[get("/politica-de-privacidade")]
pub fn privacy_policy_pt(user: User) -> Template {
    privacy_policy(user)
}

#[get("/privacy-policy")]
pub fn privacy_policy(user: User) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser, syspage};

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<(SysUser, Address, Phone)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let content: Vec<String> = syspage::table
        .select(syspage::syspage_content)
        .filter(syspage::syspage_title.eq("Privacidade".to_string()))
        .load::<String>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", (content, self_data));

    Template::render("home/privacy", context)
}

#[get("/politica-de-privacidade", rank = 2)]
pub fn privacy_policy_no_login_pt() -> Template {
    privacy_policy_no_login()
}

#[get("/privacy-policy", rank = 2)]
pub fn privacy_policy_no_login() -> Template {
    use crate::schema::syspage;
    let mut context = HashMap::new();

    let content: Vec<String> = syspage::table
        .select(syspage::syspage_content)
        .filter(syspage::syspage_title.eq("Privacidade".to_string()))
        .load::<String>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", (content, ));

    Template::render("home/privacy", &context)
}
