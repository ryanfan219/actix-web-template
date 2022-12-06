use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "SERVER_ENV", default = "dev")]
    pub server_env: String,
}

lazy_static! {
    pub static ref ARG_CONFIG: Config = Config::init_from_env().unwrap();
}

#[test]
fn test_config_can_be_loaded_from_hashmap() {
    // std::env::set_var("DB_HOST", "127.0.0.1");
    // Initialize config from a HashMap to avoid test race conditions
    let mut config = Config::init_from_env().unwrap();

    assert_eq!(config.server_env, "dev");
}
