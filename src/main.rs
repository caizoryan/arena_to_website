// ask for a channel slug
// accept an auth
//
// download channel content into memory
// parse channel content into blocks as html
mod arena_helpers;
mod headers;

use arena_helpers::*;

use arena_rs::{
    channel::{GetChannelThumb, GetChannelThumbBuilder},
    Query,
};
use build_html::*;
use headers::{ChannelThumb, Content};
use reqwest::{Client, Error};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut page = init_page();

    let slug = "what-fast-as-fuck-software".to_string();
    let client = Client::default();
    let channel_thumb = GetChannelThumbBuilder::default()
        .slug(slug)
        .build()
        .unwrap();

    let channel_thumb = channel_thumb
        .query(&client)
        .await
        .unwrap()
        .json::<ChannelThumb>()
        .await
        .unwrap();

    let content = get_channel_content(&channel_thumb, &client).await.unwrap();

    // for each content add to html page
    //
    content
        .contents
        .iter()
        .for_each(|e| add_block_to(&mut page, e));

    fs::write("test.html", page.to_html_string()).unwrap();

    Ok(())
}

fn add_block_to(page: &mut HtmlPage, block: &Content) {
    let container = match block.class {
        headers::Class::Image => get_image_container(block),
        headers::Class::Attachment => get_empty_container(),
        headers::Class::Text => get_text_container(block),
        headers::Class::Link => get_link_container(block),
        headers::Class::Channel => get_empty_container(),
        headers::Class::Media => get_empty_container(),
    };

    page.add_container(container);
}

fn get_empty_container() -> Container {
    Container::new(ContainerType::Div).into()
}

fn get_link_container(block: &Content) -> Container {
    let url = block.get_content_url();
    let title = block.title.as_ref().unwrap();

    Container::new(ContainerType::Div)
        .with_attributes(vec![("class", "arena-link")])
        .with_image(block.get_content_url(), title)
        .with_link(url, title)
        .into()
}

fn get_text_container(block: &Content) -> Container {
    let text = block.get_content();

    Container::new(ContainerType::Div)
        .with_attributes(vec![("class", "arena-text")])
        .with_raw(text)
        .into()
}

fn get_image_container(block: &Content) -> Container {
    let url = block.get_content_url();

    Container::new(ContainerType::Div)
        .with_attributes(vec![("class", "arena-image")])
        .with_image(url, "")
        .into()
}

fn init_page() -> HtmlPage {
    HtmlPage::new()
        .with_style(r#"img{width: 200px;}.arena-text{border: 1px solid black; width: 200px;}"#)
}
