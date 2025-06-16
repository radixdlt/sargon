use crate::prelude::*;
use std::{path::Path, vec};

impl BlogPostsClient {
    pub async fn fetch_cached_blog_posts(&self) -> Result<Vec<BlogPost>> {
        let bytes = self
            .file_system_client
            .load_from_file(Path::new(CACHE_BLOG_POSTS_PATH))
            .await?;

        match bytes {
            Some(bytes) => bytes.deserialize(),
            None => Ok(vec![]),
        }
    }

    pub async fn save_blog_posts_to_cache(&self, posts: &Vec<BlogPost>) {
        _ = self
            .file_system_client
            .save_to_file(
                Path::new(CACHE_BLOG_POSTS_PATH),
                posts.serialize_to_bytes().unwrap(),
                true,
            )
            .await;
    }
}
