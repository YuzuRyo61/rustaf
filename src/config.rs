use std::fs;
use std::io::{BufReader, Read};

#[derive(Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub database: ConfigDatabase,
}

#[derive(Deserialize)]
pub struct ConfigServer {
    pub address: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct ConfigDatabase {
    pub uri: String,
}


fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn get_config(config_path: String) -> Result<Config, toml::de::Error> {
    let cfg_text = read_file(config_path).unwrap();
    let cfg = toml::from_str(&cfg_text)?;

    Ok(cfg)
}
