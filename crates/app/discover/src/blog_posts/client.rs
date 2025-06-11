use gateway_client_and_api::HttpClient;

use crate::prelude::*;

const BLOG_BASE_PATH: &str = "https://www.radixdlt.com/blog/";
const CACHE_BLOG_POSTS_PATH: &str = "blog_posts.json";

pub struct BlogPostsClient {
    http_client: Arc<HttpClient>,
    file_system_client: Arc<FileSystemClient>,
}

impl BlogPostsClient {
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
                let remote_blog_posts: Vec<BlogPost> =
                    remote_blog_posts.into_iter().map(BlogPost::from).collect();
                let cached_blog_posts = cached_blog_posts.unwrap_or_default();

                self.save_blog_posts_to_cache(&remote_blog_posts).await;

                let new_blog_post =
                    remote_blog_posts.first().and_then(|blog_post| {
                        if !cached_blog_posts.contains(blog_post) {
                            Some(blog_post.clone())
                        } else {
                            None
                        }
                    });
                let blog_posts =
                    BlogPosts::new(remote_blog_posts, new_blog_post);
                return Ok(blog_posts);
            }
            Err(_) => {
                return cached_blog_posts
                    .map(|blog_posts| BlogPosts::new(blog_posts, None))
            }
        }
    }
}

use std::{path::Path, vec};

impl BlogPostsClient {
    async fn fetch_cached_blog_posts(&self) -> Result<Vec<BlogPost>> {
        let bytes = self
            .file_system_client
            .load_from_file(Path::new(CACHE_BLOG_POSTS_PATH))
            .await?;

        match bytes {
            Some(bytes) => bytes.deserialize(),
            None => Ok(vec![]),
        }
    }

    async fn save_blog_posts_to_cache(&self, posts: &Vec<BlogPost>) {
        _ = self
            .file_system_client
            .save_to_file(
                Path::new(CACHE_BLOG_POSTS_PATH),
                posts.serialize_to_bytes().unwrap(),
                true,
            )
            .await;
    }

    async fn fetch_remote_blog_posts(&self) -> Result<Vec<RemoteBlogPost>> {
        // TODO: use real url
        let request = NetworkRequest::new_get(Url::sample());
        self.http_client
            .execute_request_with_decoding(request)
            .await
    }
}

impl BlogPostsClient {
    pub fn test_client() -> Self {
        let body = Self::test_blog_posts_json();
        let http_client =
            HttpClient::new(Arc::new(MockNetworkingDriver::new(200, body)));
        let file_system_client = FileSystemClient::in_memory();
        Self::new(Arc::new(http_client), Arc::new(file_system_client))
    }

    pub fn test_blog_posts_json() -> Vec<u8> {
        let json = r#"
        [
            {
                "name": "MVP Booster Grant Winners: Rakoon.fun, DefiPlaza, Oasis",
                "image": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/68398c8b811376d5d1790651_blog%20rakoon%20oasis%20defiplaza.png",
                "slug": "mvp-booster-grant-winners-rakoon-fun-defiplaza-oasis"
            },
            {
                "name": "MVP Booster Grant Winners: RPFS, XRDegen, Liquify",
                "image": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/68304c4374aa35b75c4795d4_radix%20ecosystem%20funt%20MPV.png",
                "slug": "mvp-booster-grant-winners-2"
            },
            {
                "name": "Meet The Project: XRDegen",
                "image": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/682f201d6fef9ecf89e94bd1_meet%20the%20project-%20xrd.png",
                "slug": "meet-the-project-xrdegen"
            },
            {
                "name": "Token Holder Consultation Result",
                "image": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/682c9366d215b2b73083cfc2_concultation%20cocnluded.png",
                "slug": "token-holder-consultation-result"
            },
            {
                "name": "Token Holder Consultation Is Now Live",
                "image": "https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/681dcf8365388c9dbe4d10c0_Token%20Holder%20Consultation%20request%20for%20input.png",
                "slug": "token-holder-consultation-is-now-live"
            }
        ]
        "#;
        json.as_bytes().to_vec()
    }
}

impl From<RemoteBlogPost> for BlogPost {
    fn from(value: RemoteBlogPost) -> Self {
        let slug = &value.slug;
        Self::new(
            value.name,
            value.image,
            Url::from_str(&format!("{BLOG_BASE_PATH}/{slug}")).unwrap(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct RemoteBlogPost {
    name: String,
    image: Url,
    slug: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::path::Path;
    use std::str::FromStr;
    use std::sync::Arc;
    use url::Url;

    #[actix_rt::test]
    async fn test_remote_blog_post_decoding() {
        let json = BlogPostsClient::test_blog_posts_json();
        let remote: Vec<RemoteBlogPost> = json.deserialize().unwrap();
        let expected_posts = vec![
            RemoteBlogPost {
                name: "MVP Booster Grant Winners: Rakoon.fun, DefiPlaza, Oasis".to_owned(),
                image: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/68398c8b811376d5d1790651_blog%20rakoon%20oasis%20defiplaza.png").unwrap(),
                slug: "mvp-booster-grant-winners-rakoon-fun-defiplaza-oasis".to_owned(),
            },
            RemoteBlogPost {
                name: "MVP Booster Grant Winners: RPFS, XRDegen, Liquify".to_owned(),
                image: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/68304c4374aa35b75c4795d4_radix%20ecosystem%20funt%20MPV.png").unwrap(),
                slug: "mvp-booster-grant-winners-2".to_owned(),
            },
            RemoteBlogPost {
                name: "Meet The Project: XRDegen".to_owned(),
                image: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/682f201d6fef9ecf89e94bd1_meet%20the%20project-%20xrd.png").unwrap(),
                slug: "meet-the-project-xrdegen".to_owned(),
            },
            RemoteBlogPost {
                name: "Token Holder Consultation Result".to_owned(),
                image: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/682c9366d215b2b73083cfc2_concultation%20cocnluded.png").unwrap(),
                slug: "token-holder-consultation-result".to_owned(),
            },
            RemoteBlogPost {
                name: "Token Holder Consultation Is Now Live".to_owned(),
                image: Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/681dcf8365388c9dbe4d10c0_Token%20Holder%20Consultation%20request%20for%20input.png").unwrap(),
                slug: "token-holder-consultation-is-now-live".to_owned(),
            },
        ];
        pretty_assertions::assert_eq!(remote, expected_posts);
    }

    // Use an async test attribute from your runtime, e.g. Tokio:
    #[actix_rt::test]
    async fn test_from_remote_to_blog_post() {
        let remote = RemoteBlogPost {
            name: "Test Post".to_owned(),
            image: Url::from_str("https://example.com/test.png").unwrap(),
            slug: "test-post".to_owned(),
        };
        let blog_post: BlogPost = remote.into();
        assert_eq!(blog_post.name, "Test Post");
        assert_eq!(
            blog_post.image,
            Url::from_str("https://example.com/test.png").unwrap()
        );
        let expected_url =
            Url::from_str(&format!("{BLOG_BASE_PATH}/{}", "test-post"))
                .unwrap();
        assert_eq!(blog_post.url, expected_url);
    }

    #[actix_rt::test]
    async fn test_get_blog_posts_remote_success() {
        // Create a client using the test_client, which returns valid remote posts.
        let client = BlogPostsClient::test_client();
        let blog_posts = client
            .get_blog_posts()
            .await
            .expect("Should get blog posts");

        // The test JSON in test_client has 5 posts.
        assert_eq!(blog_posts.posts.len(), 5);

        // Since there is no existing cache on first run,
        // the first remote post should be marked as new.
        let first_post = blog_posts.posts.first().expect("At least one post");
        assert!(blog_posts.new_blog_post.is_some());
        assert_eq!(blog_posts.new_blog_post.unwrap(), first_post.clone());
    }

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
