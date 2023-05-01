pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(
        settings: crate::settings::Settings,
        test_pool: Option<sqlx::postgres::PgPool>,
    ) -> Result<Self, std::io::Error> {
        let connection_pool = if let Some(pool) = test_pool {
            pool
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
            match sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => {
                    tracing::event!(target: "sqlx",tracing::Level::ERROR, "Couldn't establish DB connection!: {:#?}", e);
                    panic!("Couldn't establish DB connection!")
                }
            }
        };

        sqlx::migrate!()
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database.");

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let listener = std::net::TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, settings).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: std::net::TcpListener,
    db_pool: sqlx::postgres::PgPool,
    settings: crate::settings::Settings,
) -> Result<actix_web::dev::Server, std::io::Error> {
    // For S3 client: create singleton S3 client
    let s3_client = actix_web::web::Data::new(configure_and_return_s3_client().await);

    // Database connection pool application state
    let pool = actix_web::web::Data::new(db_pool);

    let redis_url = std::env::var("REDIS_URL").expect("Failed to get REDIS_URL.");

    // Redis connection pool
    let cfg = deadpool_redis::Config::from_url(redis_url.clone());
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Cannot create deadpool redis.");
    let redis_pool_data = actix_web::web::Data::new(redis_pool);

    // For session
    let secret_key = actix_web::cookie::Key::from(settings.secret.hmac_secret.as_bytes());
    let redis_store = actix_session::storage::RedisSessionStore::new(redis_url.clone())
        .await
        .expect("Cannot unwrap redis session.");

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                actix_session::SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax)
                    .cookie_secure(true)
                    .cookie_name("sessionid".to_string())
                    .build(),
            )
            .service(crate::routes::health_check)
            // Authentication routes
            .configure(crate::routes::auth_routes_config)
            // Add database pool to application state
            .app_data(pool.clone())
            // Add redis pool to application state
            .app_data(redis_pool_data.clone())
            // S3 client
            .app_data(s3_client.clone())
            // Logging middleware
            .wrap(actix_web::middleware::Logger::default())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn configure_and_return_s3_client() -> crate::uploads::Client {
    // S3 configuration and client
    // Get id and secret key from the environment
    let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS key.");
    let aws_key_secret =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS secret key.");
    // build the aws cred
    let aws_cred = aws_sdk_s3::config::Credentials::new(
        aws_key,
        aws_key_secret,
        None,
        None,
        "loaded-from-custom-env",
    );
    // build the aws client
    let aws_region = aws_sdk_s3::config::Region::new(
        std::env::var("AWS_REGION").unwrap_or("eu-west-2".to_string()),
    );
    let aws_config_builder = aws_sdk_s3::config::Builder::new()
        .region(aws_region)
        .credentials_provider(aws_cred);

    let aws_config = aws_config_builder.build();
    crate::uploads::Client::new(aws_config)
}
