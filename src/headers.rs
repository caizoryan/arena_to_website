use serde::{Deserialize, Serialize};

fn defaul_false() -> Option<bool> {
    Some(false)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
/// Classes and Are.na Content Item can be, Block or Channel
pub enum Class {
    Image,
    Attachment,
    Text,
    Link,
    Channel,
    Media,
}

/// How a content item is stored locally
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Content {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub slug: Option<String>,
    pub class: Class,
    pub base_class: String,
    pub content: Option<String>,
    pub content_html: Option<String>,
    pub position: u32,
    pub source: Option<SourceData>,
    pub title: Option<String>,
    pub image: Option<ImageData>,
    pub path: Option<String>,
    pub attachment: Option<AttachmentData>,
    #[serde(default = "defaul_false")]
    pub downloaded: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SourceData {
    pub url: Option<String>,
}

impl Content {
    /// Add the path where the content item will be stored locally
    pub fn add_path(&mut self, path: String) {
        self.path = Some(path);
    }

    /// Get the path where the content item will be stored locally.
    /// This is also inclusive of the file name, when filename config is implemented
    /// will need to change this and also look here on how to restructure it
    pub fn get_path(&self) -> String {
        match self.class {
            Class::Image => self.get_image_path(),
            Class::Attachment => self.get_attachment_path(),
            Class::Text => self.get_text_path(),
            Class::Link => self.get_link_path(),
            Class::Media => self.get_link_path(),
            Class::Channel => self.get_channel_path(),
        }
    }

    /// Get the path along with the file extension
    pub fn get_attachment_path(&self) -> String {
        let id = self.id;
        let ext = self.attachment.as_ref().unwrap().extension.clone();
        let title = format!("{}.{}", id, ext);
        let path = format!("{}/{}", self.path.as_ref().unwrap(), title);
        path
    }

    /// Channel Path
    pub fn get_channel_path(&self) -> String {
        let path = format!("{}", self.path.as_ref().unwrap());
        path
    }

    /// Text path with .md extension
    pub fn get_text_path(&self) -> String {
        let id = self.id;
        let ext = "md";
        let title = format!("{}.{}", id, ext);
        let path = format!("{}/{}", self.path.as_ref().unwrap(), title);
        path
    }

    /// Link path with .url extension
    pub fn get_link_path(&self) -> String {
        let id = self.id;
        let ext = "url";
        let title = format!("{}.{}", id, ext);
        let path = format!("{}/{}", self.path.as_ref().unwrap(), title);
        path
    }

    /// Image path with applicable extension
    pub fn get_image_path(&self) -> String {
        let id = self.id;
        let ext = if self.image.as_ref().is_none() {
            self.source
                .as_ref()
                .unwrap()
                .url
                .as_ref()
                .unwrap()
                .clone()
                .split(".")
                .last()
                .unwrap()
                .to_string()
        } else {
            self.image
                .as_ref()
                .unwrap()
                .filename
                .as_ref()
                .unwrap()
                .split(".")
                .last()
                .unwrap()
                .to_string()
        };

        let title = format!("{}.{}", id, ext);
        let path = format!("{}/{}", self.path.as_ref().unwrap(), title);
        path
    }

    /// Get the url of the content item
    pub fn get_content_url(&self) -> String {
        match self.class {
            Class::Link => match self.image {
                Some(ref image) => image.display.url.clone(),
                None => "".to_string(),
            },
            Class::Image => match self.image {
                Some(ref image) => image.display.url.clone(),
                None => self.source.as_ref().unwrap().url.as_ref().unwrap().clone(),
            },
            Class::Attachment => self.attachment.as_ref().unwrap().url.clone(),
            _ => panic!("no url for this class"),
        }
    }

    /// Get the content in case a file doesn't need to be downloaded
    /// Only in case of Text, Link and Media
    pub fn get_content(&self) -> String {
        match self.class {
            Class::Text => self.content_html.as_ref().unwrap().clone(),
            Class::Link => {
                let url = self.source.as_ref().unwrap().url.as_ref().unwrap().clone();
                format!("[InternetShortcut]\nURL={}", url)
            }
            Class::Media => {
                let url = self.source.as_ref().unwrap().url.as_ref().unwrap().clone();
                format!("[InternetShortcut]\nURL={}", url)
            }
            _ => panic!("no content for this class"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub display: ImageUrl,
    pub thumb: ImageUrl,
    pub square: ImageUrl,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AttachmentData {
    pub file_name: String,
    pub file_size: u64,
    pub file_size_display: String,
    pub content_type: String,
    pub extension: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentDaddy {
    pub slug: String,
    pub contents: Vec<Content>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Channels {
    pub length: u32,
    pub channels: Vec<ChannelThumb>,
}

#[derive(Deserialize, Debug)]
pub struct BasicUser {
    pub length: u32,
    pub slug: Option<String>,
}

impl<'a> Into<ChannelThumb> for &'a Content {
    fn into(self) -> ChannelThumb {
        ChannelThumb {
            title: self.title.as_ref().unwrap().to_string(),
            id: self.id,
            slug: self.slug.as_ref().unwrap().to_string(),
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            length: 0,
            status: None,
            owner_id: None,
        }
    }
}

/// Thumbnail data for a Channel to hold important information for syncing and downloading other content
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ChannelThumb {
    pub title: String,
    pub id: u32,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
    pub length: u32,
    pub status: Option<String>,
    pub owner_id: Option<u32>,
}

impl ChannelThumb {
    pub fn get_path(&self) -> String {
        format!("channels/{}/", self.slug)
    }
}
