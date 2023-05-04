use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use once_cell::sync::Lazy;
use sqlx::Row;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = backend::telemetry::get_subscriber(false);
    backend::telemetry::init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/users/login/", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app(pool: sqlx::postgres::PgPool) -> TestApp {
    dotenv::from_filename(".env.test").ok();
    Lazy::force(&TRACING);

    let settings = {
        let mut s = backend::settings::get_settings().expect("Failed to read settings.");
        // Use a random OS port
        s.application.port = 0;
        s
    };

    let application = backend::startup::Application::build(settings.clone(), Some(pool.clone()))
        .await
        .expect("Failed to build application.");
    let address = format!("http://127.0.0.1:{}", application.port());

    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address,
        test_user: TestUser::generate().await,
        api_client: client,
    };

    test_app.test_user.store(&pool).await;

    test_app
}

pub struct TestUser {
    pub email: String,
    pub password: String,
    first_name: String,
    last_name: String,
}

impl TestUser {
    pub async fn generate() -> Self {
        Self {
            email: uuid::Uuid::new_v4().to_string(),
            password: uuid::Uuid::new_v4().to_string(),
            first_name: uuid::Uuid::new_v4().to_string(),
            last_name: uuid::Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &sqlx::postgres::PgPool) {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .expect("Unable to hash password.")
            .to_string();

        let user_id = sqlx::query(
            "INSERT INTO users (email, password, first_name, last_name, is_active, is_staff, is_superuser) 
            VALUES ($1, $2, $3, $4, true, true, true) RETURNING id"
        )
        .bind(&self.email)
        .bind(password_hash)
        .bind(&self.first_name)
        .bind(&self.last_name)
        .map(|row: sqlx::postgres::PgRow| -> uuid::Uuid{
            row.get("id")
       })
        .fetch_one(pool)
        .await
        .expect("Failed to store test user.");

        sqlx::query(
            "INSERT INTO user_profile (user_id) 
                    VALUES ($1) 
                ON CONFLICT (user_id) 
                DO NOTHING",
        )
        .bind(user_id)
        .execute(pool)
        .await
        .expect("Cannot store user_profile to the DB");
    }
}
