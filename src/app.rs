//! Application state.

use crate::feat;
use color_eyre::Result;
use feat::{
    foo::{Foo, FooImpl},
    web::ServerConfig,
};
use parking_lot::Mutex;
use std::sync::Arc;

/// The application state.
///
/// Features should expose a single structure to include in the state.
///
/// All entries in the state should either be behind an `Arc`, or be cheap to clone. The state is
/// accessed by each web endpoint.
#[derive(Debug, Clone)]
pub struct Application {
    pub foo: Arc<dyn feat::foo::Foo>,
    pub webserver: Arc<Mutex<feat::web::WebServer>>,
}

impl Application {
    /// Create a new application builder.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }
}

/// An error that may occur while building the application.
#[derive(Debug, thiserror::Error)]
#[error("application builder error")]
pub struct ApplicationBuilderError;

/// The application builder.
#[derive(Debug, Default)]
pub struct ApplicationBuilder {
    pub feature: Option<Arc<dyn Foo>>,
    pub server_config: ServerConfig,
}

impl ApplicationBuilder {
    /// Build the application.
    ///
    /// # Errors
    ///
    /// Returns an error if a feature failed to execute.
    pub fn build(self) -> Result<Application> {
        // Any features that are missing should use a default.
        Ok(Application {
            foo: self.feature.unwrap_or_else(|| Arc::new(FooImpl)),
            webserver: Arc::new(Mutex::new(
                feat::WebServer::default().with_server_config(self.server_config),
            )),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{feat::foo::StubFoo, Application};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn create_an_app_with_default_features() {
        // Given a default application
        let app = Application::builder().build().unwrap();

        // When we use the feature
        let result = app.foo.run_foo();

        // We get 0 back
        assert_eq!(result, 0);
    }

    #[test]
    #[traced_test]
    #[should_panic(expected = "stub")]
    fn create_an_app_and_substitute_the_foo_feature_with_something_else() {
        // Extension trait is imported so we can customize which `Foo` to use via `with_foo`.
        use crate::feat::foo::FooApplicationBuilderExt;

        // Given an application where the `feature` was changed to a stub
        let app = Application::builder().with_foo(StubFoo).build().unwrap();

        // When we use the feature
        app.foo.run_foo();

        // Then we panic instead of running the business logic [annotation on test function checks this]
    }
}
