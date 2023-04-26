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
        } else if settings.debug {
            get_connection_pool(&settings.database).await
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

pub async fn get_connection_pool(
    settings: &crate::settings::DatabaseSettings,
) -> sqlx::postgres::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(settings.connect_to_db())
}

async fn run(
    listener: std::net::TcpListener,
    db_pool: sqlx::postgres::PgPool,
    settings: crate::settings::Settings,
) -> Result<actix_web::dev::Server, std::io::Error> {
    // Database connection pool application state
    let pool = actix_web::web::Data::new(db_pool);

    // Redis connection pool
    let cfg = deadpool_redis::Config::from_url(settings.redis.uri);
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Cannot create deadpool redis.");
    let redis_pool_data = actix_web::web::Data::new(redis_pool);

    // For session
    let secret_key = actix_web::cookie::Key::from(settings.secret.hmac_secret.as_bytes());

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(if settings.debug {
                actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(true)
                .build()
            } else {
                actix_session::SessionMiddleware::new(
                    actix_session::storage::CookieSessionStore::default(),
                    secret_key.clone(),
                )
            })
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin(&settings.frontend_url)
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(crate::routes::health_check)
            // Authentication routes
            .configure(crate::routes::auth_routes_config)
            // Add database pool to application state
            .app_data(pool.clone())
            // Add redis pool to application state
            .app_data(redis_pool_data.clone())
            // Logging middleware
            .wrap(actix_web::middleware::Logger::default())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
