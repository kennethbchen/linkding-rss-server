use std::convert::TryInto;

use linkding::{Bookmark, ListBookmarksArgs};
use serde::Deserialize;

use rss::Channel;
use urlencoding::encode;

#[derive(Clone, Debug, Deserialize)]
pub struct Feed {
    pub title: String,
    pub description: String,
    pub route: String,
    pub allowed_tags: Option<Vec<String>>,
    pub blocked_tags: Option<Vec<String>>,
    pub unread: Option<bool>,
}

impl Feed {
    pub fn get_query_string(&self) -> Option<String> {
        let Some(tags) = self.allowed_tags.as_ref() else {
            return None;
        };

        let mut tags: Vec<String> = tags.clone();

        // URL encode each tag
        for i in 0..tags.len() {
            tags[i] = encode(&tags[i]).to_string();
        }

        return Some(tags.join("&"));
    }

    pub fn allows_bookmark(&self, bookmark: &Bookmark) -> bool {
        /*
        // Note: This is functionally unneeded because we already filter for allowed_tags
        // when we query linkding

        // If this feed has allowed tags, check if they all exist in the bookmark
        match &self.allowed_tags {
            Some(feed_tags) => {
                for feed_tag in feed_tags {
                    let mut found: bool = false;

                    for tag in &bookmark.tag_names {
                        if feed_tag == tag {
                            found = true;
                        }
                    }

                    if !found {
                        return false;
                    }
                }
            }
            None => {}
        }
        */
        match &self.blocked_tags {
            Some(feed_tags) => {
                for feed_tag in feed_tags {
                    for tag in &bookmark.tag_names {
                        if feed_tag == tag {
                            return false;
                        }
                    }
                }
            }
            None => {}
        }

        return true;
    }
}

impl TryInto<ListBookmarksArgs> for Feed {
    type Error = ();

    fn try_into(self) -> Result<ListBookmarksArgs, Self::Error> {
        return Ok(ListBookmarksArgs {
            query: self.get_query_string(),
            limit: None,
            offset: None,
            unread: self.unread,
        });
    }
}

impl TryInto<Channel> for Feed {
    type Error = ();

    fn try_into(self) -> Result<rss::Channel, Self::Error> {
        let mut channel = Channel::default();
        channel.set_title(&self.title);
        channel.set_description(&self.description);

        return Ok(channel);
    }
}
