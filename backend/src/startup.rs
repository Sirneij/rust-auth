pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build() -> Result<Self, std::io::Error> {
        let address = "127.0.0.1:5000";

        let listener = std::net::TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: std::net::TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new().service(crate::routes::health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
