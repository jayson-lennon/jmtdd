//! A feature of the applicaiton.

mod web;

use crate::feat::web::FeatureRouter;
use crate::ApplicationBuilder;
use std::sync::Arc;

/// Behaviors of the feature.
///
/// We use a trait here so that we can create test versions.
pub trait Foo: FeatureRouter + 'static + Send + Sync + std::fmt::Debug {
    fn run_foo(&self) -> i32;
}

/// The production feature implementation.
#[derive(Debug)]
pub struct FooImpl;

impl Foo for FooImpl {
    fn run_foo(&self) -> i32 {
        0
    }
}

/// Extension trait to allow customizing the feature while building the application.
pub trait FooApplicationBuilderExt {
    #[must_use]
    fn with_foo<F>(self, foo: F) -> Self
    where
        F: Foo;
}

impl FooApplicationBuilderExt for ApplicationBuilder {
    fn with_foo<F>(self, feature: F) -> Self
    where
        F: Foo,
    {
        // We just switch out the config with whatever was provided.
        Self {
            feature: Some(Arc::new(feature)),
            ..self
        }
    }
}

/// A stub Foo that panics when any behavior is called.
///
/// When testing, anything that calls the feature will panic, indicating that you will need to
/// create a fake for that particular test. This also surfaces the dependency on the feature.
#[derive(Debug)]
pub struct StubFoo;

impl Foo for StubFoo {
    fn run_foo(&self) -> i32 {
        unimplemented!("stub")
    }
}

/// A fake Foo that allows you to return a specific value.
#[derive(Debug)]
pub struct FakeFoo {
    value: i32,
}

impl FakeFoo {
    #[must_use]
    pub fn with_value(self, value: i32) -> Self {
        Self { value }
    }
}

impl Foo for FakeFoo {
    fn run_foo(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::feat::foo::{Foo, FooImpl};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn foo_returns_0_when_called() {
        // This is a "business logic" test to make sure it actually does the thing.

        let feature = FooImpl;
        assert_eq!(feature.run_foo(), 0);
    }
}
