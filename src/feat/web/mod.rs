//! The web server feature of the application.

mod feature_router;
mod healthcheck;

use crate::{Application, ApplicationBuilder};
use color_eyre::Result;
use feature_router::all_routes;
use std::{future::Future, net::SocketAddr};
use tokio::task::JoinHandle;

pub use feature_router::FeatureRouter;

/// The webserver component of the application.
#[derive(Debug, Default)]
pub struct WebServer {
    config: ServerConfig,
    /// When running in the background, this will be `Some`.
    task: Option<JoinHandle<()>>,
}

impl WebServer {
    /// Change the server config.
    #[must_use]
    pub fn with_server_config(self, config: ServerConfig) -> Self {
        Self { config, ..self }
    }
}

/// Configuration of the webserver.
#[derive(Debug)]
pub struct ServerConfig {
    /// Address of the server.
    addr: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from(([127, 0, 0, 1], 0)),
        }
    }
}

impl ServerConfig {
    /// Change the address for the server.
    #[must_use]
    pub fn with_socket_addr(self, addr: SocketAddr) -> Self {
        Self { addr }
    }
}

/// Extension trait to allow customizing the server config while building the application.
pub trait WebApplicationBuilderExt {
    /// Update the server configuration in the application builder
    #[must_use]
    fn with_server_config(self, config: ServerConfig) -> Self;
}

impl WebApplicationBuilderExt for ApplicationBuilder {
    fn with_server_config(self, config: ServerConfig) -> Self {
        Self {
            server_config: config,
            ..self
        }
    }
}

/// An error that may occur with the Web feature.
#[derive(Debug, thiserror::Error)]
#[error("a web server error occurred")]
pub struct WebError;

/// Adds a `serve` method to the application to easily start the server.
pub trait WebApplicationExt {
    /// Spin up a web server in the background.
    fn serve_background(&mut self) -> impl Future<Output = Result<()>>;
    /// Spin up a web server. This loops infinitely.
    fn serve(&mut self) -> impl Future<Output = Result<()>>;
}

impl WebApplicationExt for Application {
    async fn serve_background(&mut self) -> Result<()> {
        let listener = {
            let addr = {
                let server = self.webserver.lock();
                server.config.addr
            };
            tokio::net::TcpListener::bind(addr).await.unwrap()
        };

        let local_addr = listener.local_addr()?;

        let mut server = self.webserver.lock();
        server.config.addr = local_addr;

        let router = all_routes(self.clone());

        let server_task = tokio::spawn(async move {
            tracing::info!(bind = %local_addr, "server running");
            axum::serve(listener, router).await.unwrap();
        });

        server.task = Some(server_task);

        Ok(())
    }

    async fn serve(&mut self) -> Result<()> {
        let listener = {
            let addr = {
                let server = self.webserver.lock();
                server.config.addr
            };
            tokio::net::TcpListener::bind(addr).await.unwrap()
        };

        let local_addr = listener.local_addr()?;

        {
            let mut server = self.webserver.lock();
            server.config.addr = local_addr;
        }

        let router = all_routes(self.clone());

        tracing::info!(bind = %local_addr, "server running");
        axum::serve(listener, router).await.unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        feat::web::{ServerConfig, WebApplicationBuilderExt, WebApplicationExt},
        Application,
    };
    use hyper::StatusCode;
    use reqwest::Response;
    use std::net::SocketAddr;
    use tracing_test::traced_test;

    /// A client used for testing the server.
    #[derive(Debug)]
    pub struct TestClient {
        client: reqwest::Client,
        addr: SocketAddr,
    }

    impl Default for TestClient {
        fn default() -> Self {
            Self {
                client: reqwest::Client::default(),
                addr: SocketAddr::from(([127, 0, 0, 1], 0)),
            }
        }
    }

    impl TestClient {
        /// Make a GET request.
        pub async fn get<U>(&self, url: U) -> Result<Response, reqwest::Error>
        where
            U: Into<String>,
        {
            let url = url.into();
            let addr = self.addr;
            let url = format!("http://{addr}{url}");
            self.client.get(url).send().await
        }

        /// Create a new test client from an existing application.
        pub fn from_app(app: &Application) -> Self {
            let addr = {
                let server = app.webserver.lock();
                server.config.addr
            };
            Self {
                client: reqwest::Client::default(),
                addr,
            }
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn serves_application() {
        // Given a default application with a webserver running
        let mut app = Application::builder().build().unwrap();
        app.serve_background().await.unwrap();

        // When we make a request from the server
        let request = TestClient::from_app(&app)
            .get("/healthcheck")
            .await
            .unwrap();

        // Then the server responds with an OK status code
        assert_eq!(request.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[traced_test]
    async fn uses_custom_server_configuration() {
        // Given an application with a customized bind address and a running webserver
        let addr = SocketAddr::from(([127, 0, 0, 1], 65501));
        let mut app = Application::builder()
            .with_server_config(ServerConfig::default().with_socket_addr(addr))
            .build()
            .unwrap();
        app.serve_background().await.unwrap();

        // When we make a request to the specified address
        let request = TestClient::from_app(&app)
            .get("/healthcheck")
            .await
            .unwrap();

        // Then the server responds with an OK status code
        assert_eq!(request.status(), StatusCode::OK);
    }
}
