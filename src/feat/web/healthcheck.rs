//! A "health check" route so the server status can be probed.

use axum::{routing::get, Router};
use hyper::StatusCode;

/// Router for performing a healthcheck.
pub fn healthcheck() -> Router {
    Router::new().route("/healthcheck", get(|| async { StatusCode::OK }))
}

#[cfg(test)]
mod tests {
    use crate::{
        feat::web::{tests::TestClient, WebApplicationExt},
        Application,
    };
    use hyper::StatusCode;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn health_check_returns_200_ok() {
        // Given a default application with a webserver running
        let mut app = Application::builder().build().unwrap();
        app.serve_background().await.unwrap();

        // When we make a health check request
        let request = TestClient::from_app(&app)
            .get("/healthcheck")
            .await
            .unwrap();

        // Then the server responds with an OK status code
        assert_eq!(request.status(), StatusCode::OK);
    }
}
