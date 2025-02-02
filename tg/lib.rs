mod schema;
pub use schema::*;

mod addons;
pub use addons::*;

pub mod custom;
pub use custom::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(all(not(feature = "client")))]
mod bare;
#[cfg(all(not(feature = "client")))]
pub use bare::*;

#[cfg(feature = "serializer")]
pub mod serializer;

#[macro_export]
macro_rules! method {
	($name:ty, $id:literal, $response:ty) => {
		impl crate::Method for $name {
			const NAME: &str = $id;
			type Response = $response;
		}
	};
}