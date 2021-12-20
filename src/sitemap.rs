use std::fs::{write};
use sitemap::writer::SiteMapWriter;
use chrono::{Utc, DateTime, Datelike, FixedOffset, NaiveDate};
use sitemap::structs::{ChangeFreq, UrlEntry};
use diesel::prelude::*;
use diesel::dsl::sql_query;
use diesel::sql_types::{Text};
use serde::Serialize;

/* Struct for post_slug list */
#[derive(QueryableByName, Serialize)]
struct Post {
    #[sql_type = "Text"]
    post_slug: String
}
/* Struct for tarologist_slug list */
#[derive(QueryableByName, Serialize)]
struct Tarologist {
    #[sql_type = "Text"]
    tarologist_slug: String
}

pub fn sitemap_generate(website: &str, lang: &str) {
    let mut output = Vec::<u8>::new();
    let mut blog_term: &str = "blog-post";
    let mut tarologist_term: &str = "tarologist";
    let static_routes: Vec<&str> = match lang {
        "PT" => {
            blog_term = "artigos";
            tarologist_term = "tarologo";
            vec![
                "cadastre-se",
                "entrar",
                "comprar-creditos",
                "tarologos",
                "perguntas-frequentes",
                "quem-somos",
                "artigos",
                "depoimentos",
                "politica-de-privacidade",
                "contato",
            ]
        }
        _ => {
            vec![
                "register",
                "login",
                "buy-credits",
                "tarologists",
                "frequently-asked-question",
                "about-us",
                "blog",
                "testimonials",
                "privacy-policy",
                "contact",
            ]
        }
    };
    
    let sitemap_writer = SiteMapWriter::new(&mut output);

    let mut urlwriter = sitemap_writer
            .start_urlset()
            .expect("Unable to write urlset");

    let today = Utc::now();
    let (_is_common_era, year) = today.year_ce();
    let date = DateTime::from_utc(
        NaiveDate::from_ymd(year as i32, today.month(), today.day()).and_hms(0, 0, 0),
        FixedOffset::east(0),
    );

    let home_entry = UrlEntry::builder()
            .loc(website)
            .changefreq(ChangeFreq::Weekly)
            .lastmod(date)
            .priority(1.00)
            .build()
            .expect("valid");
    urlwriter.url(home_entry).expect("Unable to write url");

    for route in static_routes.iter() {
        let static_url = format!("{}/{}", website, route);
        let url_entry = UrlEntry::builder()
            .loc(static_url)
            .changefreq(ChangeFreq::Weekly)
            .lastmod(date)
            .priority(0.80)
            .build()
            .expect("valid");

        urlwriter.url(url_entry).expect("Unable to write url");
    }

    /* Tarologists data slug urls  */
    match sql_query(
        "select REPLACE(LOWER(clerk_info.clerk_info_exhibition ), ' ', '-' ) as tarologist_slug from public.sysuser 
            inner join clerk_info on clerk_info.user_id = sysuser.user_id where user_status = true"
    )
    .get_results::<Tarologist>(&crate::establish_connection()) {
        Ok(tarologists) => {
            if tarologists.len() > 0 {
                for tarologist in tarologists.iter() {
                    let static_url = format!("{}/{}/{}", website, tarologist_term, tarologist.tarologist_slug);
                    let url_entry = UrlEntry::builder()
                        .loc(static_url)
                        .changefreq(ChangeFreq::Weekly)
                        .lastmod(date)
                        .priority(0.75)
                        .build()
                        .expect("valid");

                    urlwriter.url(url_entry).expect("Unable to write url");
                }
            }
        }
        Err(_) => {}
    }

    /* blog data slug urls  */
    match sql_query(
        "SELECT REPLACE(LOWER(post.post_title), ' ', '-' ) as post_slug FROM post"
    )
    .get_results::<Post>(&crate::establish_connection()) {
        Ok(posts) => {
            if posts.len() > 0 {
                for post in posts.iter() {
                    let static_url = format!("{}/{}/{}", website, blog_term, post.post_slug);
                    let url_entry = UrlEntry::builder()
                        .loc(static_url)
                        .changefreq(ChangeFreq::Weekly)
                        .lastmod(date)
                        .priority(0.65)
                        .build()
                        .expect("valid");

                    urlwriter.url(url_entry).expect("Unable to write url");
                }
            }
        }
        Err(_) => {}
    }

    let _sitemap_writer = urlwriter.end().expect("close the urlset block");
    
    match write("assets/sitemap.xml", &output) {
        Ok(s) => { println!("sitemap.xml successfully generated: {:?}", s)}
        Err(e) => { println!("sitemap.xml generation error: {:?}", e) }
    }

}