use std::io::{File, IoError};
use std::io::net::ip::IpAddr;

use serialize::Encodable;

use toml;

use error::FerrumResult;
use error::ErrorKind::{DecodingError, InvalidConfigError};

/// Representation of configuration data for the server
#[deriving(Decodable, Encodable)]
pub struct Config {
    // IP on which to listen
    ip_addr: String,
    // Port on which to listen
    port: u16,
    // Asset directory path
    asset_path: String,
    // Relative path to static files from asset directory
    static_path: String,
    // Relative path to content files from asset directory
    content_path: String,
}

impl Config {
    pub fn new(path_str: &str) -> FerrumResult<Config> {
        let path = Path::new(path_str);
        let contents = try!(File::open(&path).read_to_end());
        let contents = String::from_utf8_lossy(contents.as_slice());
        match toml::decode_str(contents.as_slice()) {
            Some(v) => Ok(v),
            None => panic!((DecodingError(path_str.to_string()), "Failed to decode file"))
        }
    }

    pub fn ip_addr(&self) -> FerrumResult<IpAddr> {
        match from_str::<IpAddr>(self.ip_addr.as_slice()) {
            Some(ip) => Ok(ip),
            None => Err(panic!((InvalidConfigError, "Failed to parse IP address in config.")))
        }
    }

    pub fn port(&self) -> u16 { self.port }
    pub fn asset_path(&self) -> &str { self.asset_path.as_slice() }
    pub fn static_path(&self) -> String { format!("{}/{}", self.asset_path, self.static_path) }
    pub fn content_path(&self) -> String { format!("{}/{}", self.asset_path, self.content_path) }
}

/// Creates a default config file.
pub fn write_default_config(path_str: &str) -> Result<(), IoError> {
    let path = Path::new(path_str);
    info!("Writing default config.");
    let config = default_config();
    let encoded = toml::encode_str(&config);
    let mut file = File::create(&path);
    file.write(encoded.into_bytes().as_slice())
}

/// Default config values
pub fn default_config() -> Config {
    Config {
        ip_addr: "127.0.0.1".to_string(),
        port: 3000,
        asset_path: "assets".to_string(),
        static_path: "static".to_string(),
        content_path: "content".to_string()
    }
}
