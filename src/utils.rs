use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pokeapi=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
