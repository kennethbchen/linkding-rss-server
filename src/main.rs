use linkding::LinkDingClient;

use serde::Deserialize;

use core::panic;
use std::fs;

use actix_web::{App, HttpServer, get, web};

mod feed;
use feed::Feed;

mod linkding_bridge;
use linkding_bridge::{LinkdingArgs, build_channel};

#[derive(Debug, Deserialize)]
struct AppConfig {
    linkding_args: LinkdingArgs,
    feeds: Vec<Feed>,
}

impl AppConfig {
    fn get_feed_from_route(&self, route: &String) -> Option<Feed> {
        for feed in &self.feeds {
            if feed.route == route.as_str() {
                return Some(feed.clone());
            }
        }
        return None;
    }
}

struct AppState {
    config: AppConfig,
    linkding_client: LinkDingClient,
}

#[get("/{feed_route}")]
async fn get_feed_rss(data: web::Data<AppState>, path: web::Path<String>) -> Option<String> {
    let feed_route: String = path.into_inner();

    let Some(feed) = &data.config.get_feed_from_route(&feed_route) else {
        return None;
    };

    let Ok(channel) = build_channel(&feed, &data.linkding_client) else {
        return None;
    };
    return Some(channel.to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    HttpServer::new(|| {
        // Read config file
        let Ok(config) = fs::read_to_string("config.toml") else {
            panic!("config.toml could not be read");
        };

        let config: AppConfig = toml::from_str(&config).unwrap();

        let client: LinkDingClient =
            LinkDingClient::new(&config.linkding_args.url, &config.linkding_args.api_key);

        App::new()
            .app_data(web::Data::new(AppState {
                config: config,
                linkding_client: client,
            }))
            .service(get_feed_rss)
    })
    .bind(("127.0.0.1", 10100))?
    .run()
    .await
}
