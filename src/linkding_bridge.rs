use linkding::{Bookmark, LinkDingClient, ListBookmarksArgs, ListBookmarksResponse};

use rss::{Channel, Item};

use serde::Deserialize;

use crate::feed::Feed;

#[derive(Debug, Deserialize)]
pub struct LinkdingArgs {
    pub api_key: String,
    pub url: String,
}

pub struct FeedError;

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

pub fn build_channel(feed: &Feed, linkding: &LinkDingClient) -> Result<Channel, FeedError> {
    let args: ListBookmarksArgs = feed.clone().try_into().unwrap();

    let Ok(response) = linkding.list_bookmarks(args) else {
        return Err(FeedError);
    };

    let items: Vec<Bookmark> = filter_bookmarks(response, feed);
    let items: Vec<Item> = bookmarks_to_items(items);

    let mut channel: Channel = feed.clone().try_into().unwrap();

    channel.set_items(items);
    return Ok(channel);
}
