use linkding::{Bookmark, LinkDingClient, LinkDingError, ListBookmarksArgs, ListBookmarksResponse};

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

fn filter_bookmarks(response: ListBookmarksResponse, feed: &Feed) -> Vec<Bookmark> {
    // TODO: handle response.next
    let mut bookmarks: Vec<Bookmark> = Vec::new();

    for item in response.results {
        if feed.allows_bookmark(&item) {
            bookmarks.push(item);
        }
    }
    return bookmarks;
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

    let args: ListBookmarksArgs = config.feeds[0].clone().try_into().unwrap();

    println!("{:#?}", feed);
    let Ok(response) = client.list_bookmarks(args) else {
        panic!();
    };

    let bookmarks: Vec<Bookmark> = filter_bookmarks(response, &feed);

    println!("{:#?}", bookmarks);
}
