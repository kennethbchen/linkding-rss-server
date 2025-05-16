use linkding::LinkDingClient;

use serde::Deserialize;

use core::panic;
use std::fs;

mod feed;
use feed::Feed;

mod linkding_bridge;
use linkding_bridge::{LinkdingArgs, build_channel};

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

    let client: LinkDingClient =
        LinkDingClient::new(&config.linkding_args.url, &config.linkding_args.api_key);

    let feed = config.feeds[0].clone();

    let Ok(channel) = build_channel(&feed, &client) else {
        panic!("Feed error occured");
    };

    println!("{:#?}", channel.to_string());
}
