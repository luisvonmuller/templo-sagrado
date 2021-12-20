pub mod admin;

/* Template */
use rocket_contrib::templates::Template;

/* Table macros */
use diesel::prelude::*;

/* Date implementations */
use chrono::{Utc};

/* Session Handling */
use crate::User;

/* Importing form mod */
use rocket::request::LenientForm;

#[get("/depoimento/<clerk_id>")]
pub fn give_testimonial(user: User, clerk_id: i32) -> Template {
    use crate::models::SysUser;
    use crate::schema::sysuser;

    let mut context = std::collections::HashMap::new();

    let self_data = sysuser::table
        .select(sysuser::all_columns)
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<SysUser>(&crate::establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", (self_data, clerk_id));

    Template::render("home/testimonial", &context)
}

#[derive(Debug, FromForm)]
pub struct NewTestiomonial {
    pub testimonial_value: i32,
    pub testimonial_content: String,
    pub testimonial_clerk_id: i32,
}

#[post("/submit-testimonial", data = "<testimonial_data>")]
pub fn submit_testiomonial(user: User, testimonial_data: LenientForm<NewTestiomonial>) {
    use crate::models::NewTestimonials;
    use crate::schema::testimonials;
    use diesel::prelude::*;

    diesel::insert_into(testimonials::table)
        .values(NewTestimonials {
            testimonials_clerk_id: testimonial_data.testimonial_clerk_id,
            testimonials_client_id: user.user_id as i32,
            testimonials_content: testimonial_data.testimonial_content.to_string(),
            testimonials_value: testimonial_data.testimonial_value,
            testimonials_date: Utc::today().naive_utc(),
            testimonials_status: false,
        })
        .execute(&crate::establish_connection())
        .expect("Shit happned");
}