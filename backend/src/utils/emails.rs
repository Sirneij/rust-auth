use lettre::AsyncTransport;

#[tracing::instrument(
    name = "Generic e-mail sending function.",
    skip(
        recipient_email,
        recipient_first_name,
        recipient_last_name,
        subject,
        html_content,
        text_content
    ),
    fields(
        recipient_email = %recipient_email,
        recipient_first_name = %recipient_first_name,
        recipient_last_name = %recipient_last_name
    )
)]
pub async fn send_email(
    sender_email: Option<String>,
    recipient_email: String,
    recipient_first_name: String,
    recipient_last_name: String,
    subject: impl Into<String>,
    html_content: impl Into<String>,
    text_content: impl Into<String>,
) -> Result<(), String> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let email = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                "Auth Systems with SvelteKit",
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
            [recipient_first_name, recipient_last_name].join(" "),
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

    // Open a remote connection to gmail
    let mailer: lettre::AsyncSmtpTransport<lettre::Tokio1Executor> =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&settings.email.host)
            .unwrap()
            .credentials(creds)
            .build();

    // Send the email
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

#[tracing::instrument(
    name = "Generic multipart e-mail sending function.",
    skip(redis_connection),
    fields(
        recipient_user_id = %user_id,
        recipient_email = %recipient_email,
        recipient_first_name = %recipient_first_name,
        recipient_last_name = %recipient_last_name
    )
)]
pub async fn send_multipart_email(
    subject: String,
    user_id: uuid::Uuid,
    recipient_email: String,
    recipient_first_name: String,
    recipient_last_name: String,
    template_name: &str,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
) -> Result<(), String> {
    let settings = crate::settings::get_settings().expect("Unable to load settings.");
    let title = subject.clone();

    let issued_token = match crate::utils::issue_confirmation_token_pasetors(
        user_id,
        redis_connection,
        None,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
            return Err(format!("{}", e));
        }
    };
    let web_address = {
        if settings.debug {
            format!(
                "{}:{}",
                settings.application.base_url, settings.application.port,
            )
        } else {
            settings.application.base_url
        }
    };
    let confirmation_link = {
        if template_name == "password_reset_email.html" {
            format!(
                "{}/users/password-change/confirm/change-password/?token={}",
                web_address, issued_token,
            )
        } else {
            format!(
                "{}/users/register/confirm/?token={}",
                web_address, issued_token,
            )
        }
    };
    let current_date_time = chrono::Local::now();
    let dt = current_date_time + chrono::Duration::minutes(settings.secret.token_expiration);

    let template = crate::ENV.get_template(template_name).unwrap();
    let ctx = minijinja::context! {
        title => &title,
        confirmation_link => &confirmation_link,
        domain => &settings.frontend_url,
        expiration_time => &settings.secret.token_expiration,
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
        recipient_first_name,
        recipient_last_name,
        subject,
        html_text,
        text,
    ));
    Ok(())
}
