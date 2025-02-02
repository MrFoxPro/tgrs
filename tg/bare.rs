pub trait Method: serde::Serialize + Sized + Send + 'static {
	const NAME: &str;
	type Response: serde::de::DeserializeOwned + Send + 'static;
}
