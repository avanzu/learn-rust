use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
};

// use bcrypt::DEFAULT_COST;
use dotenv::dotenv;
use log::info;
use argon2::Config; 
use rand::Rng;

pub fn port_of(s: String) -> u16 {
    s.parse::<u16>().expect(&format!("`{}` is not numeric", &s))
}

pub fn bind_addr(var: &str, default_port: u16) -> impl ToSocketAddrs {
    dotenv().ok();
    SocketAddr::from((
        [0, 0, 0, 0],
        env::var(var).map(port_of).unwrap_or(default_port),
    ))
}

pub fn database_url(default_url: String) -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap_or(default_url)
}

pub fn password_salt() -> Vec<u8> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    
    Vec::from(salt)
}

pub fn hash_password_salted(raw: &String) -> Result<String, argon2::Error>  {
    let config = Config::default(); 
    info!("Generating salt"); 
    let salt: [u8; 32] = rand::thread_rng().gen();
    info!("hasing password using default config");
    let hash = argon2::hash_encoded(raw.as_bytes(), &salt, &config)?;
    info!("hasing done, returning hash and salt");
    Ok(hash)
}

pub fn hash_password(raw: String, salt: &Vec<u8>) -> Result<String, argon2::Error> {
    info!("Hashing password");

    let config = Config::default(); 
    let hash = argon2::hash_encoded(raw.as_bytes(), salt, &config)?;

    info!("Password hash complete"); 

    Ok(hash)
}

pub fn compare_password (password: &String, hash: &str) -> Result<bool, argon2::Error> {
    info!("Comparing password");
    Ok(argon2::verify_encoded(hash, password.as_bytes())?)
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_port() {
        let port = port_of(String::from("1024"));
        assert_eq!(1024, port)
    }
    #[test]
    #[should_panic(expected = "`foo-bar` is not numeric")]
    fn parse_port_nonum() {
        port_of(String::from("foo-bar"));
    }
}
