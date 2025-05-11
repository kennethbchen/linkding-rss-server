use std::convert::TryInto;

use linkding::ListBookmarksArgs;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Feed {
    pub route: String,
    pub allowed_tags: Option<Vec<String>>,
    pub blocked_tags: Option<Vec<String>>,
    pub unread: Option<bool>,
}

impl Feed {
    pub fn get_query_string(&self) -> Option<String> {
        let tags: Vec<String> = self.allowed_tags.as_ref().unwrap().clone();

        //println!("{:#?}", self.allowed_tags.as_ref().unwrap());
        return Some(tags.join("&"));
    }
}

impl TryInto<linkding::ListBookmarksArgs> for Feed {
    type Error = ();

    fn try_into(self) -> Result<linkding::ListBookmarksArgs, Self::Error> {
        return Ok(ListBookmarksArgs {
            query: self.get_query_string(),
            limit: None,
            offset: None,
            unread: self.unread,
        });
    }
}
