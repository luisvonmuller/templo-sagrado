use futures::future::lazy;
use futures::stream::Stream;
use new_tokio_smtp::error::GeneralError;
use new_tokio_smtp::send_mail::{EncodingRequirement, Mail, MailAddress, MailEnvelop};
use new_tokio_smtp::{command, Connection, ConnectionConfig, Domain};

/* may this work? */
use handlebars::Handlebars;
use std::collections::HashMap;

pub fn send_mail(mails: Vec<MailEnvelop>) {
    let config = ConnectionConfig::builder(Domain::from_unchecked("smtp.gmail.com"))
    .expect("resolving domain failed")
    .auth(
        command::auth::Plain::from_username(
            MailAddress::from_unchecked("templosagrado.marketing@gmail.com").clone(),
            String::from("templo-vonmuller-2021"),
        )
        .expect("username/password can not contain \\0 bytes"),
    )
    .build();

    let mails = mails
        .into_iter()
        .map(|m| -> Result<_, GeneralError> { Ok(m) });

    tokio::run(lazy(move || {
        Connection::connect_send_quit(config, mails)
            .then(|result| Ok(result))
            .for_each(|result| {
                if let Err(err) = result {
                    println!("[sending mail failed]: {}", err);
                } else {
                    println!("[successfully send mail]")
                }
                Ok(())
            })
    }))
}

/* This function stands for notifying users that somebody got online */
pub fn notify_user_online(
    user_name: String,
    user_mail: String,
    clerk_name: String,
    clerk_slug: String,
) {
    use chrono::Utc;
    use std::fs;

    /* Can be a vector for a lot of users */
    let send_to = MailAddress::from_unchecked(user_mail);

    let mail_stuff = fs::read_to_string("templates/mail-notify.html.hbs")
        .expect("Something went wrong reading the file");

    let mut handlebars = Handlebars::new();

    // register the template. The template string will be verified and compiled.
    let mut context = HashMap::new();
    context.insert("mail_data", (&user_name, clerk_name, clerk_slug));

    handlebars
        .register_template_string("mail-notify", mail_stuff)
        .unwrap();

    let raw_mail = format!(
        concat!(
            "Date: <{}>\r\n",
            "From: Templo Sagrado <{}>\r\n",
            "Subject: {}, O atendente que você mostrou interesse está disponível AGORA! \r\n",
            "To: <{}>\r\n",
            "Content-Type: text/html; charset='UTF-8' \r\n",
            "Content-Transfer-Encoding: quoted-printable \r\n",
            "{}"
        ),
        Utc::now().naive_utc(),
        "templosagrado.marketing@gmail.com",
        user_name.as_str(),
        send_to.as_str(),
        handlebars.render("mail-notify", &context).unwrap()
    );

    /* If we set a requiring encoding, the smtp server of destination must implement it too,
    so at all its better to just don't. */
    let mail_data = Mail::new(EncodingRequirement::Mime8bit, raw_mail.to_owned());

    let mail = MailEnvelop::new(
        MailAddress::from_unchecked("templosagrado.marketing@gmail.com"),
        vec1![send_to],
        mail_data,
    );

    send_mail(vec![mail]);
}

/* This function stands for replying of an clerk */
pub fn answer_mail(
    subject: String,
    mail_content: String,
    mail_dest: String,
    clerk_name: String,
    clerk_slug: String,
) {
    use chrono::Utc;
    use std::fs;

    /* Can be a vector for of a lot of users */
    let send_to = MailAddress::from_unchecked(mail_dest);
    let mut handlebars = Handlebars::new();

    let mail_stuff = fs::read_to_string("templates/mail-answer.html.hbs")
        .expect("Something went wrong reading the file");

    /* register the template. The template string will be verified and compiled. */
    let mut context = HashMap::new();
    context.insert("mail_data", (mail_content, clerk_name, clerk_slug));

    handlebars
        .register_template_string("mail-notify", mail_stuff)
        .unwrap();

    let raw_mail = format!(
        concat!(
            "Date: <{}>\r\n",
            "From: Templo Sagrado <{}>\r\n",
            "Subject: {} \r\n",
            "To: <{}>\r\n",
            "Content-Type: text/html; charset='UTF-8' \r\n",
            "Content-Transfer-Encoding: quoted-printable \r\n",
            "{} \r\n\r\n E-mail de atendimento (Está salvo em sua conta para visualização posterior) "
        ),
        Utc::now().naive_utc(),
        "templosagrado.marketing@gmail.com",
        subject.as_str(),
        send_to.as_str(),
        handlebars.render("mail-notify", &context).unwrap()
    );

    /* If we set a requiring encoding, the smtp server of destination must implement it too,
    so at all its better to just don't. */
    let mail_data = Mail::new(EncodingRequirement::Mime8bit, raw_mail.to_owned());

    let mail = MailEnvelop::new(
        MailAddress::from_unchecked("templosagrado.marketing@gmail.com"),
        vec1![send_to],
        mail_data,
    );

    send_mail(vec![mail]);
}


pub fn pass_mail(mail_content: String, mail_dest: String) { 
    use chrono::Utc;
    use std::fs;

    /* Can be a vector for of a lot of users */
    let send_to = MailAddress::from_unchecked(mail_dest);
    let mut handlebars = Handlebars::new();

    let mail_stuff = fs::read_to_string("templates/mail-pass.html.hbs")
        .expect("Something went wrong reading the file");

    /* register the template. The template string will be verified and compiled. */
    let mut context = HashMap::new();
    context.insert("mail_data", mail_content);

    handlebars
        .register_template_string("mail-pass", mail_stuff)
        .unwrap();

    let raw_mail = format!(
        concat!(
            "Date: <{}>\r\n",
            "From: Templo Sagrado <{}>\r\n",
            "Subject: Sua nova senha \r\n",
            "To: <{}>\r\n",
            "Content-Type: text/html; charset='UTF-8' \r\n",
            "Content-Transfer-Encoding: quoted-printable \r\n",
            "{} \r\n\r\n Sugerimos que mude a sua snha o quanto antes. "
        ),
        Utc::now().naive_utc(),
        "templosagrado.marketing@gmail.com",
        send_to.as_str(),
        handlebars.render("mail-pass", &context).unwrap()
    );

    /* If we set a requiring encoding, the smtp server of destination must implement it too,
    so at all its better to just don't. */
    let mail_data = Mail::new(EncodingRequirement::Mime8bit, raw_mail.to_owned());

    let mail = MailEnvelop::new(
        MailAddress::from_unchecked("templosagrado.marketing@gmail.com"),
        vec1![send_to],
        mail_data,
    );

    send_mail(vec![mail]);
}