use crate::prelude::*;
use std::path::Path;

impl BlogPostsClient {
    pub async fn fetch_cached_blog_posts(
        &self,
    ) -> Result<Option<CachedBlogPosts>> {
        let bytes = self
            .file_system_client
            .load_from_file(Path::new(CACHE_BLOG_POSTS_PATH))
            .await?;

        match bytes {
            Some(bytes) => {
                let posts: CachedBlogPosts = bytes.deserialize()?;
                Ok(Some(posts))
            }
            None => Ok(None),
        }
    }

    pub async fn save_blog_posts_to_cache(
        &self,
        posts: &Vec<BlogPost>,
        last_update: Timestamp,
    ) {
        let to_save = CachedBlogPosts {
            posts: posts.clone(),
            last_update,
        };
        _ = self
            .file_system_client
            .save_to_file(
                Path::new(CACHE_BLOG_POSTS_PATH),
                to_save.serialize_to_bytes().unwrap(),
                true,
            )
            .await;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedBlogPosts {
    pub posts: Vec<BlogPost>,
    pub last_update: Timestamp,
}

impl CachedBlogPosts {
    pub fn new(posts: Vec<BlogPost>, last_update: Timestamp) -> Self {
        Self { posts, last_update }
    }
}
