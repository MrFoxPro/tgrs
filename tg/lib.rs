mod schema;
pub use schema::*;

mod addons;
pub use addons::*;

pub mod custom;
pub use custom::*;

#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "reqwest")]
pub use reqwest::*;

#[cfg(all(not(feature = "reqwest")))]
mod bare;
#[cfg(all(not(feature = "reqwest")))]
pub use bare::*;

#[cfg(feature = "reqwest")]
mod serializer;
#[cfg(feature = "reqwest")]
pub use serializer::*;

#[macro_export]
macro_rules! method {
	($name:ty, $id:literal, $response:ty) => {
		impl crate::Method for $name {
			const NAME: &str = $id;
			type Response = $response;
		}
	};
}