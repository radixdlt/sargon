use crate::prelude::*;

pub struct BlogPostsClient {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) file_system_client: Arc<FileSystemClient>,
}

impl BlogPostsClient {
    pub fn new_with_drivers(
        networking_driver: Arc<dyn NetworkingDriver>,
        file_system_driver: Arc<dyn FileSystemDriver>,
    ) -> Self {
        Self::new(
            Arc::new(HttpClient::new(networking_driver)),
            Arc::new(FileSystemClient::new(file_system_driver)),
        )
    }

    pub fn new(
        http_client: Arc<HttpClient>,
        file_system_client: Arc<FileSystemClient>,
    ) -> Self {
        Self {
            http_client,
            file_system_client,
        }
    }

    pub async fn get_blog_posts(&self) -> Result<BlogPosts> {
        let remote_blog_posts = self.fetch_remote_blog_posts().await;
        let cached_blog_posts = self.fetch_cached_blog_posts().await;

        match remote_blog_posts {
            Ok(remote_blog_posts) => {
                let cached_blog_posts = cached_blog_posts.unwrap_or_default();

                self.save_blog_posts_to_cache(&remote_blog_posts).await;

                let new_blog_post =
                    remote_blog_posts.first().and_then(|blog_post| {
                        if !cached_blog_posts.is_empty()
                            && !cached_blog_posts.contains(blog_post)
                        {
                            Some(blog_post.clone())
                        } else {
                            None
                        }
                    });
                let blog_posts =
                    BlogPosts::new(remote_blog_posts, new_blog_post);
                return Ok(blog_posts);
            }
            Err(error) => {
                return cached_blog_posts
                .and_then(|blog_posts| {
                    if blog_posts.is_empty() {
                        Err(error) 
                    } else {
                        Ok(BlogPosts::new(blog_posts, None))
                    }
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::str::FromStr;
    use std::sync::Arc;
    use url::Url;

    #[actix_rt::test]
    async fn test_get_blog_posts_remote_failure_fallback_to_cache() {
        // Prepare an in-memory file system client with cached posts.
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        let cached_posts = vec![BlogPost::new(
            "Cached Post".to_owned(),
            Url::from_str("https://example.com/cached.png").unwrap(),
            Url::from_str(&format!("{BLOG_BASE_PATH}/cached-post")).unwrap(),
        )];

        // Save the cached posts.
        file_system_client
            .save_to_file(
                Path::new(CACHE_BLOG_POSTS_PATH),
                cached_posts.serialize_to_bytes().unwrap(),
                true,
            )
            .await
            .unwrap();

        // Simulate a remote fetch failure by using a networking driver that returns an error.
        let http_client = Arc::new(HttpClient::new(Arc::new(
            MockNetworkingDriver::new(500, Vec::new()),
        )));
        let client =
            BlogPostsClient::new(http_client, file_system_client.clone());

        // The failure should force a fallback to cache.
        let blog_posts = client
            .get_blog_posts()
            .await
            .expect("Fallback to cached posts");
        // With fallback, new_blog_post is not set.
        assert_eq!(blog_posts.posts.len(), 1);
        assert!(blog_posts.new_blog_post.is_none());
        assert_eq!(blog_posts.posts[0], cached_posts[0]);
    }

    #[actix_rt::test]
    async fn test_save_and_load_cached_blog_posts() {
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        // Create a dummy http_client (won't be used in this test).
        let http_client = Arc::new(HttpClient::new(Arc::new(
            MockNetworkingDriver::new(200, Vec::new()),
        )));

        let client =
            BlogPostsClient::new(http_client, file_system_client.clone());
        let posts = vec![BlogPost::new(
            "Test Cached".to_owned(),
            Url::from_str("https://example.com/cached.png").unwrap(),
            Url::from_str(&format!("{BLOG_BASE_PATH}/test-cached")).unwrap(),
        )];

        client.save_blog_posts_to_cache(&posts).await;
        let loaded_posts = client.fetch_cached_blog_posts().await.unwrap();
        assert_eq!(loaded_posts, posts);
    }
}
