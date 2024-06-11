use envconfig::Envconfig;

#[derive(Debug, Envconfig)]
pub(crate) struct AppConfig {
    #[envconfig(from = "HOST", default = "0.0.0.0")]
    pub host: String,

    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,
}
