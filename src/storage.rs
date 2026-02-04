use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    city: City 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    id: u16
}

pub fn write_config(id: &str) {
    if let Some(mut conf) = dirs::config_dir() {
        conf.push("jsh");

        let jsh_config = conf.join("jsh.toml");
        if !conf.exists() {
            fs::create_dir_all(&conf).unwrap();
        }

        let id = id.parse().unwrap();

        let config = Config {
            city: City { id }
        };

        let toml_string = toml::to_string(&config).unwrap();
        fs::write(jsh_config, toml_string).unwrap();
    } else {
        let mut config_path = PathBuf::new();
        if let Some(usr) = std::env::home_dir()
            && let Some(home) = usr.to_str() 
        {
            config_path.push(format!("{}/.config/jsh", home));
        }

        fs::create_dir_all(&config_path).expect("Error: Cannot create config directory");
        
        let jsh_config = config_path.join("jsh.toml");

        let id = id.parse().unwrap();
        let config = Config {
            city: City { id }
        };

        let toml_string = toml::to_string(&config).unwrap();
        fs::write(jsh_config, toml_string).expect("Error: Can't write to config");
    }
}

pub fn read_config() -> u16 {
    if let Some(mut conf) = dirs::config_dir() {
        conf.push("jsh");
        let jsh_config = conf.join("jsh.toml");

        if !jsh_config.exists() {
            0
        } else {
            let content = fs::read_to_string(jsh_config).expect("Error: Can't read config");
            let config: Config = toml::from_str(&content).expect("Error: Can't parse config");
            config.city.id
        }
    } else {
        0
    }
}
