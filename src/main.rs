use config::Config;

use linkding::{LinkDingClient, LinkDingError, ListBookmarksArgs};

mod feed;

use feed::Feed;

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

    let test_feed: Feed = Feed {
        route: "temp".to_string(),
        allowed_tags: Some(vec!["youtube".to_string(), "video".to_string()]),
        blocked_tags: None,
        unread: None,
    };

    println!("{}", test_feed.get_query_string());
    /*
    let client: LinkDingClient = LinkDingClient::new(&cfg.url, &cfg.api_key);

    let args: ListBookmarksArgs = ListBookmarksArgs {
        query: Some("youtube".to_string()),
        limit: None,
        offset: None,
        unread: None,
    };

    let response = client.list_bookmarks(args);

    match response {
        Ok(res) => {
            println!("{}", res.count);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    */
}
