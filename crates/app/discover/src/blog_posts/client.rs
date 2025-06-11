use gateway_client_and_api::HttpClient;

use crate::prelude::*;

const BLOG_BASE_PATH: &str = "https://www.radixdlt.com/blog/";

pub struct BlogPostsClient {
    http_client: Arc<HttpClient>,
}

impl BlogPostsClient {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    pub async fn get_blog_posts(&self) -> Result<Vec<BlogPost>> {
        let remote_blog_posts = self.fetch_remote_blog_posts().await?;
        Ok(remote_blog_posts.into_iter().map(BlogPost::from).collect())
    }
}

impl BlogPostsClient {
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
        let body = json.serialize_to_bytes().unwrap();
        let http_client =
            HttpClient::new(Arc::new(MockNetworkingDriver::new(200, body)));

        Self::new(Arc::new(http_client))
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
