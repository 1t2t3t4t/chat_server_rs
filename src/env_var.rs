use std::env;

pub struct EnvVar {
    pub port: Option<String>,
}

pub fn get_env() -> EnvVar {
    EnvVar {
        port: env::var("PORT").ok(),
    }
}
