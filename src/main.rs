use config::Config;

struct AppConfig {
    api_key: String,
    url: String,
}

fn parse_config() -> AppConfig {
    let Ok(config) = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
    else {
        panic!("config.toml not found");
    };

    let Ok(api_key) = &config.get_string("api_key") else {
        panic!("'api_key' not found in config");
    };
    let api_key: String = api_key.clone();

    let Ok(url) = &config.get_string("url") else {
        panic!("'url' not found in config");
    };
    let url: String = url.clone();

    return AppConfig { api_key, url };
}

fn main() {
    let cfg: AppConfig = parse_config();

    println!("{}", cfg.api_key);

    println!("{}", cfg.url);
}
