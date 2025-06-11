use crate::prelude::*;

use sargon::BlogPostsClient as InternalBlogPostsClient;

#[derive(uniffi::Object)]
struct BlogPostsClient {
    wrapped: Arc<InternalBlogPostsClient>,
}

#[uniffi::export]
impl BlogPostsClient {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            wrapped: Arc::new(InternalBlogPostsClient::new()),
        }
    }
}

#[uniffi::export]
impl BlogPostsClient {
    pub async fn get_blog_posts(&self) -> Result<Vec<BlogPost>> {
        self.wrapped.get_blog_posts().await.into_iter_result()
    }
}
