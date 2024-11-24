//! Web integratino for the Foo feature

use super::Foo;
use crate::{feat::FeatureRouter, Application};
use axum::{extract::State, routing::get, Router};

/// Implement the same route for all Foos
impl<F> FeatureRouter for F
where
    F: Foo,
{
    fn router(&self, app: Application) -> Router {
        Router::new().route("/foo", get(handler)).with_state(app)
    }
}

/// Handler should be responsible for transforming incoming and outgoing data. Logic should be
/// delegated to the feature itself.
async fn handler(State(app): State<Application>) -> String {
    let foo = app.foo.run_foo();
    format!("{foo}")
}

#[cfg(test)]
mod tests {
    use crate::{
        feat::{
            foo::{FakeFoo, FooApplicationBuilderExt},
            web::{tests::TestClient, WebApplicationExt},
        },
        Application,
    };
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn handles_foo() {
        // Given an application running a webserver
        let mut app = Application::builder()
            // substitute the real Foo service with one that returns `10`
            .with_foo(FakeFoo::default().with_value(10))
            .build()
            .unwrap();
        app.serve_background().await.unwrap();

        // When we make a request to /foo
        let request = TestClient::from_app(&app).get("/foo").await.unwrap();

        // Then the server responds with the correct data
        assert_eq!(request.text().await.unwrap(), format!("10"));
    }
}
