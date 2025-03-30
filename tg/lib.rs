#![feature(assert_matches, default_field_values)]

pub mod client;

mod addons;
pub use addons::*;

pub mod custom;
pub use custom::*;

mod schema;
pub use schema::*;

#[cfg(test)]
mod test;