use axum::{extract::State, routing::get, Router};

use crate::{feat::FeatureRouter, Application};

use super::Foo;

/// Implement the same route for all Foos
impl<F> FeatureRouter for F
where
    F: Foo,
{
    fn router(&self, app: Application) -> Router {
        Router::new().route("/foo", get(handler)).with_state(app)
    }
}

/// Handler has full application access. It should be minimal (just returning the `foo` value
/// as a String, in this case).
async fn handler(State(app): State<Application>) -> String {
    let foo = app.foo.run_foo();
    format!("{foo}")
}
