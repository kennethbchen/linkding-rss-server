pub struct Feed {
    pub route: String,
    pub allowed_tags: Option<Vec<String>>,
    pub blocked_tags: Option<Vec<String>>,
    pub unread: Option<bool>,
}

impl Feed {
    pub fn get_query_string(&self) -> String {
        let tags: String;

        todo!("not yet implemented");
    }
}
