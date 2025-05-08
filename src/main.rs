use config::Config;

struct AppSettings {
    api_key: String,
    url: String,
}

fn parse_config() {
    let Ok(config) = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
    else {
        panic!("config.toml not found");
    };

    let Ok(api_key) = &config.get_string("api_key") else {
        panic!("'api_key' not found in config");
    };

    let Ok(url) = &config.get_string("url") else {
        panic!("'url' not found in config");
    };
    println!("{}", api_key)
}

fn main() {
    parse_config();
}
