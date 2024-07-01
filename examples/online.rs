pub use vrchatapi::apis;

fn main() {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));

    let me = apis::authentication_api::get_current_user(&config).unwrap();

    if let Some(username) = me.username {
        println!("Username: {}", username);
    } else {
        println!("Could not fetch username");
    }

    let online = apis::system_api::get_current_online_users(&config).unwrap();
    println!("Current Online Users: {}", online);
}