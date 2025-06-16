use crate::prelude::*;

use sargon::BlogPostsClient as InternalBlogPostsClient;

#[derive(uniffi::Object)]
struct BlogPostsClient {
    wrapped: Arc<InternalBlogPostsClient>,
}

#[uniffi::export]
impl BlogPostsClient {
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        file_system_driver: Arc<dyn FileSystemDriver>,
    ) -> Self {
        Self {
            wrapped: Arc::new(InternalBlogPostsClient::new_with_drivers(
                Arc::new(NetworkingDriverAdapter {
                    wrapped: networking_driver,
                }),
                Arc::new(FileSystemDriverAdapter {
                    wrapped: file_system_driver,
                }),
            )),
        }
    }
}

#[uniffi::export]
impl BlogPostsClient {
    pub async fn get_blog_posts(&self) -> Result<BlogPosts> {
        self.wrapped.get_blog_posts().await.into_result()
    }
}
