use tracing_subscriber::layer::SubscriberExt;

pub fn get_subscriber(debug: bool) -> impl tracing::Subscriber + Send + Sync {
    let env_filter = if debug {
        "trace".to_string()
    } else {
        "info".to_string()
    };
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(env_filter));

    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(stdout_log);

    let json_log = if !debug {
        let json_log = tracing_subscriber::fmt::layer().json();
        Some(json_log)
    } else {
        None
    };

    subscriber.with(json_log)
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
