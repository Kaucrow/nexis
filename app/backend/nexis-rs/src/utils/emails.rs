use crate::prelude::*;
use lettre::AsyncTransport;

pub async fn send_email(
    sender_email: Option<String>,
    recipient_email: String,
    recipient_name: String,
    subject: impl Into<String>,
    html_content: impl Into<String>,
    text_content: impl Into<String>,
) -> Result<(), String> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let email = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                "Joemama",
                if sender_email.is_some() {
                    sender_email.unwrap()
                } else {
                    settings.email.host_user.clone()
                }
            )
            .parse()
            .unwrap(),
        )
        .to(format!(
            "{} <{}>",
            //[recipient_first_name, recipient_last_name].join(" "),
            recipient_name,
            recipient_email
        )
        .parse()
        .unwrap())
        .subject(subject)
        .multipart(
        lettre::message::MultiPart::alternative()
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_PLAIN)
                        .body(text_content.into()),
                )
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_HTML)
                        .body(html_content.into()),
                ),
        )
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        settings.email.host_user,
        settings.email.host_user_password,
    );

    // open a remote connection to gmail
    let mailer: lettre::AsyncSmtpTransport<lettre::Tokio1Executor> =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&settings.email.host)
            .unwrap()
            .credentials(creds)
            .build();
    
    match mailer.send(email).await {
        Ok(_) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "Email successfully sent!");
            Ok(())
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Could not send email: {:#?}", e);
            Err(format!("Could not send email: {:#?}", e))
        }
    }
}

pub async fn send_multipart_email(
    subject: String,
    user_id: ObjectId,
    recipient_email: String,
    recipient_name: String,
    template_filename: &str,
    redis_pool: &deadpool_redis::Pool,
) -> Result<(), String> {
    // can be optimized by making the settings static
    let settings = crate::settings::get_settings().expect("Unable to load settings.");
    let title = subject.clone();

    let issued_token = match crate::utils::issue_confirmation_token(
        user_id,
        &redis_pool,
        None,
    )
    .await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
            return Err(format!("{}", e));
        }
    };
    let web_address = {
        if settings.debug {
            /*format!(
                "{}:{}",
                settings.application.base_url, settings.application.port,
            )*/
            &settings.frontend_url
        } else {
            //settings.application.base_url
            &settings.frontend_url
        }
    };
    let confirmation_link = {
        if template_filename == "password_reset_email.html" {
            format!(
                "{}/auth/password/verify/change_password?token={}",
                web_address, issued_token,
            )
        } else {
            format!(
                "{}/auth/register/verify/{}",
                web_address, issued_token,
            )
        }
    };
    let curr_date_time = chrono::Local::now();
    let dt = curr_date_time + chrono::Duration::minutes(settings.secret.email_token_expiration as i64);

    let template = crate::ENV.get_template(template_filename).unwrap();
    let ctx = minijinja::context! {
        title => &title,
        confirmation_link => &confirmation_link,
        domain => &settings.frontend_url,
        expiration_time => &settings.secret.email_token_expiration,
        exact_time => &dt.format("%A %B %d, %Y at %r").to_string()
    };
    let html_text = template.render(ctx).unwrap();

    let text = format!(
        r#"
        Tap the link below to confirm your email address.
        {}
        "#,
        confirmation_link
    );
    tokio::spawn(send_email(
        None,
        recipient_email,
        recipient_name,
        subject,
        html_text,
        text,
    ));
    Ok(())
}