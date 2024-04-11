use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn front_config() -> &'static FrontConfig {
	static INSTANCE: OnceLock<FrontConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		FrontConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct FrontConfig {
	// -- Crypt
	pub SITE_NAME: String,
}

impl FrontConfig {
	fn load_from_env() -> lib_utils::envs::Result<FrontConfig> {
		Ok(FrontConfig {
			// -- Crypt
			SITE_NAME: get_env("SITE_NAME")?,
		})
	}
}
