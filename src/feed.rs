use std::convert::TryInto;

use linkding::ListBookmarksArgs;
use serde::Deserialize;

use urlencoding::encode;

#[derive(Clone, Debug, Deserialize)]
pub struct Feed {
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
