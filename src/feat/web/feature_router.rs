//! Enables features to be exposed by the server.

use crate::Application;
use axum::Router;

/// Required to implement when you want to integrate a feature into the web server.
pub trait FeatureRouter {
    fn router(&self, app: Application) -> Router;
}

/// All routes for the application.
///
/// Add new `merge` declarations here as features are added.
#[allow(clippy::needless_pass_by_value)]
pub fn all_routes(app: Application) -> Router {
    Router::new()
        .merge(super::healthcheck::healthcheck())
        .merge(app.foo.router(app.clone()))
}
