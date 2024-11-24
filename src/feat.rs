//! Application features should have a module here.

pub mod foo;
pub mod web;

pub use web::{FeatureRouter, WebServer};
