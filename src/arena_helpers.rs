use crate::headers::*;
use arena_rs::user::*;
use futures::stream::{self, StreamExt};
use reqwest::{Client, Error};

use arena_rs::channel::{GetChannel, GetChannelBuilder};
use arena_rs::*;

/// A struct to hold data to build an arena_rs Get request for
/// User Channels and Channel Content
struct Request {
    slug: Slug,
    page: u32,
    per: u32,
    direction: Direction,
}

/// Will make a request(s) to the are.na API to get all the contents of a channel
pub async fn get_channel_content(
    channel: &ChannelThumb,
    client: &Client,
) -> Result<ContentDaddy, Error> {
    let per = 100.0 as f32;
    let pages = channel.length as f32 / per;
    let pages = f32::ceil(pages) as u32;

    let mut requests: Vec<Request> = Vec::new();

    for i in 1..=pages {
        requests.push(Request {
            slug: Slug::Slug(channel.slug.to_string()),
            page: i,
            per: per as u32,
            direction: Direction::DESC,
        });
    }

    let stream = stream::iter(requests)
        .map(|c| get_content(c, &client))
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    let flat_contents = stream
        .iter()
        .map(|c| match c {
            Ok(c) => c.contents.clone(),
            Err(_e) => Vec::new(),
        })
        .flatten()
        .collect::<Vec<_>>();

    let results = ContentDaddy {
        slug: channel.slug.clone(),
        contents: flat_contents,
    };

    Ok(results)
}

// TODO what to do when a request fails?
// TODO add a retry mechanism
// TODO add a timeout mechanism
/// Executes a get_content request to the are.na API provided a Request struct
async fn get_content(request: Request, client: &Client) -> Result<ContentDaddy, Error> {
    let request_client = GetChannelBuilder::default()
        .slug(request.slug)
        .per(request.per)
        .page(request.page)
        .direction(request.direction)
        .build()
        .unwrap();
    let response = request_client.query(client).await?;

    match response.status().is_success() {
        true => response.json::<ContentDaddy>().await,
        false => {
            println!("error at page {} and per {}", request.page, request.per);
            Err(response.error_for_status().unwrap_err())
        }
    }
}
