use crate::prelude::*;
use serde::Deserializer;

impl BlogPostsClient {
    pub async fn fetch_remote_blog_posts(&self) -> Result<Vec<BlogPost>> {
        let url = Url::from_str(BLOG_POSTS_URL).unwrap();
        let request = NetworkRequest::new_get(url);
        self.http_client
            .execute_request_with_decoding(request)
            .await
            .map(|remote_blog_posts: RemoteBlogPosts| {
                remote_blog_posts
                    .items
                    .into_iter()
                    .map(BlogPost::from)
                    .collect()
            })
    }

    pub async fn fetch_blog_posts_collection_details(
        &self,
    ) -> Result<BlogPostsCollectionDetails> {
        let url = Url::from_str(BLOG_POSTS_DETAILS_URL).unwrap();
        let request = NetworkRequest::new_get(url);
        self.http_client
            .execute_request_with_decoding(request)
            .await
    }
}

impl From<RemoteBlogPost> for BlogPost {
    fn from(value: RemoteBlogPost) -> Self {
        let slug = &value.field_data.slug;
        Self::new(
            value.field_data.name,
            value.field_data.image.url,
            Url::from_str(&format!("{BLOG_BASE_PATH}/{slug}")).unwrap(),
        )
    }
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPostsCollectionDetails {
    #[serde(deserialize_with = "deserialize_timestamp_truncated_to_seconds")]
    pub last_updated: Timestamp,
}

impl BlogPostsCollectionDetails {
    pub fn new(last_updated: Timestamp) -> Self {
        Self { last_updated }
    }
}

impl Default for BlogPostsCollectionDetails {
    fn default() -> Self {
        Self {
            last_updated: Timestamp::UNIX_EPOCH,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct RemoteBlogPosts {
    pub items: Vec<RemoteBlogPost>,
}

#[cfg(test)]
impl RemoteBlogPosts {
    pub fn new(items: Vec<RemoteBlogPost>) -> Self {
        Self { items }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteBlogPost {
    field_data: FieldData,
}

impl RemoteBlogPost {
    pub fn new(field_data: FieldData) -> Self {
        Self { field_data }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct FieldData {
    name: String,
    slug: String,
    image: Image,
}

impl FieldData {
    pub fn new(name: String, slug: String, image_url: Url) -> Self {
        Self {
            name,
            slug,
            image: Image::new(image_url),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
struct Image {
    url: Url,
}

impl Image {
    fn new(url: Url) -> Self {
        Self { url }
    }
}

fn deserialize_timestamp_truncated_to_seconds<'de, D>(
  deserializer: D,
) -> Result<Timestamp, D::Error>
where
  D: Deserializer<'de>,
{
  let timestamp: Timestamp = Timestamp::deserialize(deserializer)?;
  Ok(timestamp_truncated_to_seconds(timestamp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_last_update_decoding() {
        //let timestamp: DateTime<Utc> = "2025-06-10T09:02:35.944Z".parse().unwrap();
        // let t2: Timestamp = "2025-06-10T09:02:35.944Z".parse().unwrap();
        let json = r#"
      {
        "lastUpdated":"2025-06-10T09:02:35.944Z"
      }
      "#;
        let encoded = json.as_bytes().to_vec();
        let decoded: BlogPostsCollectionDetails =
            encoded.deserialize().unwrap();
        print!("{:?}", decoded);
    }

    #[actix_rt::test]
    async fn test_remote_blog_posts_decoding() {
        let json = r#"
       {
  "items": [
    {
      "id": "684aea4bb182475e8c7bb0c7",
      "cmsLocaleId": null,
      "lastPublished": "2025-06-12T15:41:31.751Z",
      "lastUpdated": "2025-06-12T15:41:31.751Z",
      "createdOn": "2025-06-12T14:55:07.263Z",
      "isArchived": false,
      "isDraft": false,
      "fieldData": {
        "show-table-of-contents": false,
        "featured-post": true,
        "use-video-over-image": false,
        "name": "It's Live! RedStone Oracles Now Powering Next-Generation DeFi on Radix ",
        "blog-category": [
          "649aa8a9681ec6168a57de39",
          "649aa8a9681ec6168a57de41"
        ],
        "blog-author": "649aa8a9681ec6168a57de33",
        "image": {
          "fileId": "684ae9d9f9cf5f22a9e070d6",
          "url": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/684ae9d9f9cf5f22a9e070d6_redstone%20blog%20new%20.png",
          "alt": null
        },
        "slug": "its-live-redstone-oracles-now-powering-next-generation-defi-on-radix"
      }
    },
    {
      "id": "684ad5c1ad20a9327e58ebce",
      "cmsLocaleId": null,
      "lastPublished": null,
      "lastUpdated": "2025-06-12T13:33:15.982Z",
      "createdOn": "2025-06-12T13:27:29.062Z",
      "isArchived": false,
      "isDraft": true,
      "fieldData": {
        "show-table-of-contents": false,
        "featured-post": false,
        "use-video-over-image": false,
        "name": "Meet The Project: Stabilis",
        "slug": "meet-the-project-stabilis",
        "image": {
          "fileId": "684ad55190cb758365b450fe",
          "url": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/684ad55190cb758365b450fe_meet%20the%20project%20stabilis.png",
          "alt": null
        },
        "blog-author": "649aa8a9681ec6168a57de33",
        "blog-category": [
          "649aa8a9681ec6168a57de41",
          "649aa8a9681ec6168a57de39"
        ]
      }
    }
  ],
  "pagination": {
    "limit": 2,
    "offset": 0,
    "total": 569
  }
}
        "#;

        let encoded = json.as_bytes().to_vec();
        let expected_blog_posts = vec![
            RemoteBlogPost {
                field_data: FieldData {
                    name: "It's Live! RedStone Oracles Now Powering Next-Generation DeFi on Radix ".to_owned(), 
                    slug: "its-live-redstone-oracles-now-powering-next-generation-defi-on-radix".to_owned(), 
                    image: Image {
                        url: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/684ae9d9f9cf5f22a9e070d6_redstone%20blog%20new%20.png").unwrap()
                    },
                }
            },
            RemoteBlogPost {
                field_data: FieldData {
                    name: "Meet The Project: Stabilis".to_owned(), 
                    slug: "meet-the-project-stabilis".to_owned(), 
                    image: Image {
                        url: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/684ad55190cb758365b450fe_meet%20the%20project%20stabilis.png").unwrap()
                    },
                }
            },
        ];
        let expected_remote_blog_posts = RemoteBlogPosts {
            items: expected_blog_posts,
        };
        let decoded: RemoteBlogPosts = encoded.deserialize().unwrap();
        pretty_assertions::assert_eq!(expected_remote_blog_posts, decoded);
    }
}
