use linkding::{Bookmark, LinkDingClient, LinkDingError, ListBookmarksArgs, ListBookmarksResponse};

mod feed;
use feed::Feed;

use rss::{Channel, Item};
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

fn bookmarks_to_items(bookmarks: Vec<Bookmark>) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();

    for bookmark in bookmarks {
        let mut item = Item::default();
        item.set_title(bookmark.title);
        item.set_link(bookmark.url);
        item.set_description(bookmark.description);

        item.set_pub_date(bookmark.date_added);

        items.push(item);
    }

    return items;
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

    let Ok(response) = client.list_bookmarks(args) else {
        panic!();
    };

    let bookmarks: Vec<Bookmark> = filter_bookmarks(response, &feed);

    let bookmarks: Vec<Item> = bookmarks_to_items(bookmarks);

    //println!("{:#?}", bookmarks);

    let mut channel: Channel = config.feeds[0].clone().try_into().unwrap();
    channel.set_items(bookmarks);

    println!("{:#?}", channel.to_string());
}
