use std::{env, fmt};

// Defaults.
pub const ENV_PREFIX: &str = "CARDMOD_";
pub const DEFAULT_SERVER_PORT: &str = "8000";
pub const DEFAULT_ENVIRONMENT: Environment = Environment::Local;

// Defined environment names.
pub enum Environment {
    Dev,
    Prod,
    Local,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Dev => write!(f, "dev"),
            Environment::Prod => write!(f, "prod"),
            Environment::Local => write!(f, "local"),
        }
    }
}

// Configuration holder.
pub struct Config {

    // Environment name.
    pub environment: Environment,

    // Server specific attributes.
    pub server: Server,

}

// Server configuration
pub struct Server {

    // Port to listen on.
    pub port: String,

}

// Creates a new configuration instance.
pub fn new() -> Config {

    // new: load environment name
    let environment ;
    let res = env::var(format!("{}ENVIRONMENT", ENV_PREFIX));
    match res {
        Ok(env) => {
            match env.as_str() {
                "dev" => {
                    environment = Environment::Dev;
                }
                "prod" => {
                    environment = Environment::Prod;
                }
                "local" => {
                    environment = Environment::Local;
                }
                _ => {
                    environment = DEFAULT_ENVIRONMENT;
                }
            }
        }        
        _ => {
            environment = DEFAULT_ENVIRONMENT;
        }
    };

    // new: load server port
    let server_port;
    let res = env::var(format!("{}SERVER_PORT", ENV_PREFIX));
    match res {
        Ok(port) => {
            server_port = port;
        }
        _ => {
            server_port = DEFAULT_SERVER_PORT.to_string();
        }
    };

    // new: return constructed 
    Config{
        environment: environment,
        server: Server {
            port: server_port,
        }
    }

}