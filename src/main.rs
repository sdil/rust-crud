use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};
// use migration::{Migrator, MigratorTrait};
use tokio::signal;

use crate::route::create_router;

mod handlers;
mod model;
mod route;
mod schema;

pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.expect("Couldnot connect to DB");
    let app = create_router(Arc::new(AppState { db: db.clone() }));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server started at localhost:3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to isntall Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
    _ = ctrl_c => {println!("ctrl+c")},
    _ = terminate => {println!("terminate")},
    }
}
