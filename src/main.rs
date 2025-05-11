use linkding::{LinkDingClient, LinkDingError, ListBookmarksArgs};

mod feed;
use feed::Feed;

use serde::Deserialize;

use core::panic;
use std::fs;

#[derive(Debug, Deserialize)]
struct LinkdingArgs {
    api_key: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    linkding_args: LinkdingArgs,
    feeds: Vec<Feed>,
}
fn main() {
    // Read file
    let Ok(config) = fs::read_to_string("config.toml") else {
        panic!("config.toml could not be read");
    };

    let config: AppConfig = toml::from_str(&config).unwrap();

    println!("{:#?}", config);

    let client: LinkDingClient =
        LinkDingClient::new(&config.linkding_args.url, &config.linkding_args.api_key);

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
}
