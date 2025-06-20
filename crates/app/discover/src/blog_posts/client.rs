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
        let remote_last_update = self
            .fetch_blog_posts_collection_details()
            .await
            .unwrap_or_default()
            .last_updated;
        let maybe_cached = self.fetch_cached_blog_posts().await.unwrap_or(None);

        if let Some(cached) = maybe_cached {
            // Return cached if up-to-date
            if cached.last_update >= remote_last_update {
                return Ok(BlogPosts::new(cached.posts, None));
            }
            // Attempt refresh; fallback to existing cache on error
            match self.refresh_from_remote(remote_last_update).await {
                Ok(fresh_posts) => {
                    let new_item = fresh_posts
                        .first()
                        .and_then(|post| if !cached.posts.contains(post) { Some(post.clone()) } else { None });
                    return Ok(BlogPosts::new(fresh_posts, new_item));
                }
                Err(_) => return Ok(BlogPosts::new(cached.posts, None)),
            }
        }
        // No cache: fetch and return
        let fresh = self.refresh_from_remote(remote_last_update).await?;
        Ok(BlogPosts::new(fresh, None))
    }

    async fn refresh_from_remote(
        &self,
        last_update_time: Timestamp,
    ) -> Result<Vec<BlogPost>> {
        let remote_blog_posts = self.fetch_remote_blog_posts().await?;
        self.save_blog_posts_to_cache(&remote_blog_posts, last_update_time)
            .await;
        Ok(remote_blog_posts)
    }
}

#[cfg(test)]
mod tests {
    use crate::blog_posts::client_caching::CachedBlogPosts;
    use crate::blog_posts::client_remote_fetch::{
        BlogPostsCollectionDetails, FieldData,
        RemoteBlogPost, RemoteBlogPosts,
    };

    use super::*;
    use iso8601_timestamp::Duration;
    use std::sync::Arc;
    use std::vec;
    use url::Url;

    #[actix_rt::test]
    async fn failure_empty_cache_remote_failure() {
        let file_system_client = FileSystemClient::in_memory();
        let http_client = HttpClient::new(Arc::new(
            MockNetworkingDriver::new_always_failing(),
        ));
        let sut = BlogPostsClient::new(
            Arc::new(http_client),
            Arc::new(file_system_client),
        );

        let result = sut.get_blog_posts().await;
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn success_empty_cache_remote_success() {
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        let last_updated_time = Timestamp::now_utc();
        print!("Now {:?}", last_updated_time);
        let blog_post_details =
            BlogPostsCollectionDetails::new(last_updated_time);

        let blog_posts =
            RemoteBlogPosts::new(vec![RemoteBlogPost::new(FieldData::new(
                "Test Post".to_string(),
                "test-post".to_string(),
                Url::from_str("https://example.com/image.jpg").unwrap(),
            ))]);

        let network_driver = MockNetworkingDriver::new_with_bodies(
            200,
            vec![
                blog_post_details.serialize_to_bytes().unwrap().into(),
                blog_posts.serialize_to_bytes().unwrap().into(),
            ],
        );
        let http_client = HttpClient::new(Arc::new(network_driver));

        let sut = BlogPostsClient::new(
            Arc::new(http_client),
            file_system_client.clone(),
        );

        let result = sut.get_blog_posts().await.unwrap();
        let expected_blog_posts = BlogPosts::new(
            blog_posts.items.into_iter().map(From::from).collect(),
            None,
        );
        pretty_assertions::assert_eq!(result, expected_blog_posts);
        let cache = file_system_client
            .load_from_file(CACHE_BLOG_POSTS_PATH)
            .await
            .unwrap();
        pretty_assertions::assert_eq!(
            cache.is_some(),
            true,
            "Expected to have data in cache after remote fetch"
        );
        let cached_blog_posts: CachedBlogPosts =
            cache.unwrap().deserialize().unwrap();
        let expected_cached_blog_posts = CachedBlogPosts::new(
            expected_blog_posts.posts,
            timestamp_truncated_to_seconds(last_updated_time),
        );
        pretty_assertions::assert_eq!(
            cached_blog_posts,
            expected_cached_blog_posts
        );
    }

    #[actix_rt::test]
    async fn existing_cache_no_remote_updates() {
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        let last_updated_time = Timestamp::now_utc();
        print!("Now {:?}", last_updated_time);
        let blog_post_details =
            BlogPostsCollectionDetails::new(last_updated_time);
        let cached_blog_posts = CachedBlogPosts::new(
            BlogPosts::sample().posts.clone(),
            timestamp_truncated_to_seconds(last_updated_time),
        );

        let network_driver = MockNetworkingDriver::new_with_bodies(
            200,
            vec![blog_post_details.serialize_to_bytes().unwrap().into()],
        );
        file_system_client
            .save_to_file(
                CACHE_BLOG_POSTS_PATH,
                cached_blog_posts.serialize_to_bytes().unwrap(),
                true,
            )
            .await
            .unwrap();

        let http_client = HttpClient::new(Arc::new(network_driver));

        let sut = BlogPostsClient::new(
            Arc::new(http_client),
            file_system_client.clone(),
        );

        let result = sut.get_blog_posts().await.unwrap();
        let expected_blog_posts =
            BlogPosts::new(BlogPosts::sample().posts, None);
        pretty_assertions::assert_eq!(result, expected_blog_posts);
    }

    #[actix_rt::test]
    async fn existing_cache_remote_updates_exist_failed_to_fetch_new_blog_posts(
    ) {
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        let last_updated_time = Timestamp::now_utc();
        print!("Now {:?}", last_updated_time);
        let blog_post_details = BlogPostsCollectionDetails::new(
            last_updated_time.saturating_add(Duration::seconds(10)),
        );
        let cached_blog_posts = CachedBlogPosts::new(
            BlogPosts::sample().posts.clone(),
            timestamp_truncated_to_seconds(last_updated_time),
        );

        let network_driver = MockNetworkingDriver::new_with_bodies(
            200,
            vec![blog_post_details.serialize_to_bytes().unwrap().into()],
        );
        file_system_client
            .save_to_file(
                CACHE_BLOG_POSTS_PATH,
                cached_blog_posts.serialize_to_bytes().unwrap(),
                true,
            )
            .await
            .unwrap();

        let http_client = HttpClient::new(Arc::new(network_driver));

        let sut = BlogPostsClient::new(
            Arc::new(http_client),
            file_system_client.clone(),
        );

        let result = sut.get_blog_posts().await.unwrap();
        let expected_blog_posts =
            BlogPosts::new(BlogPosts::sample().posts, None);
        pretty_assertions::assert_eq!(result, expected_blog_posts);
    }

    #[actix_rt::test]
    async fn existing_cache_remote_updates_exist_refresh_with_new_blog_posts() {
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        let last_updated_time = Timestamp::now_utc();
        let blog_post_details = BlogPostsCollectionDetails::new(
            last_updated_time.saturating_add(Duration::seconds(10)),
        );

        let old_blog_post = RemoteBlogPost::new(FieldData::new(
            "old".to_string(),
            "old-blog-post".to_string(),
            Url::from_str("https://example.com/image_old.jpg").unwrap(),
        ));
        let cached_blog_posts = CachedBlogPosts::new(
            vec![old_blog_post.clone().into()],
            timestamp_truncated_to_seconds(last_updated_time),
        );

        let new_blog_post = RemoteBlogPost::new(FieldData::new(
            "new".to_string(),
            "new-blog-post".to_string(),
            Url::from_str("https://example.com/image_new.jpg").unwrap(),
        ));

        let new_remote_blog_posts = RemoteBlogPosts::new(vec![
            new_blog_post.clone(),
            old_blog_post.clone(),
        ]);

        let network_driver = MockNetworkingDriver::new_with_bodies(
            200,
            vec![
                blog_post_details.serialize_to_bytes().unwrap().into(),
                new_remote_blog_posts.serialize_to_bytes().unwrap().into(),
            ],
        );
        file_system_client
            .save_to_file(
                CACHE_BLOG_POSTS_PATH,
                cached_blog_posts.serialize_to_bytes().unwrap(),
                true,
            )
            .await
            .unwrap();
        let http_client = HttpClient::new(Arc::new(network_driver));

        let sut = BlogPostsClient::new(
            Arc::new(http_client),
            file_system_client.clone(),
        );

        let blog_posts = sut.get_blog_posts().await.unwrap();

        let expected_blog_posts = BlogPosts::new(
            new_remote_blog_posts
                .items
                .into_iter()
                .map(From::from)
                .collect(),
            Some(new_blog_post.into()),
        );

        pretty_assertions::assert_eq!(blog_posts, expected_blog_posts);

        let new_cached_blog_posts = file_system_client
            .load_from_file(CACHE_BLOG_POSTS_PATH)
            .await
            .unwrap();

        pretty_assertions::assert_eq!(
            new_cached_blog_posts.is_some(),
            true,
            "Expected to have data in cache after remote fetch"
        );
        let new_cached_blog_posts: CachedBlogPosts =
            new_cached_blog_posts.unwrap().deserialize().unwrap();
        let expected_cached_blog_posts = CachedBlogPosts::new(
            expected_blog_posts.posts,
            timestamp_truncated_to_seconds(blog_post_details.last_updated),
        );
        pretty_assertions::assert_eq!(
            new_cached_blog_posts,
            expected_cached_blog_posts
        );
    }
}
