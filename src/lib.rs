//! A sample web application.

#![warn(clippy::pedantic)]
#![allow(clippy::default_constructed_unit_structs)]
#![allow(clippy::disallowed_names)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod feat;

pub use app::{Application, ApplicationBuilder};
