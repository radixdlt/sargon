use crate::prelude::*;

const BLOG_BASE_PATH: &str = "https://www.radixdlt.com/blog/";

pub struct BlogPostsClient {}

impl BlogPostsClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_blog_posts(&self) -> Result<Vec<BlogPost>> {
        // Temporary stub to unblock the wallet development
        Ok(
            vec! [
            BlogPost::new(
                "MVP Booster Grant Winners: Rakoon.fun, DefiPlaza, Oasis".to_owned(),
                Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/68398c8b811376d5d1790651_blog%20rakoon%20oasis%20defiplaza.png").unwrap(),
                Url::from_str(&(BLOG_BASE_PATH.to_owned() + "mvp-booster-grant-winners-rakoon-fun-defiplaza-oasis")).unwrap()
            ),
            BlogPost::new(
                "MVP Booster Grant Winners: RPFS, XRDegen, Liquify".to_owned(),
                Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/68304c4374aa35b75c4795d4_radix%20ecosystem%20funt%20MPV.png").unwrap(),
                Url::from_str(&(BLOG_BASE_PATH.to_owned() + "mvp-booster-grant-winners-2")).unwrap()
            ),
            BlogPost::new(
                "Meet The Project: XRDegen".to_owned(),
                Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/682f201d6fef9ecf89e94bd1_meet%20the%20project-%20xrd.png").unwrap(),
                Url::from_str(&(BLOG_BASE_PATH.to_owned() + "meet-the-project-xrdegen")).unwrap()
            ),
            BlogPost::new(
                "Token Holder Consultation Result".to_owned(),
                Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/682c9366d215b2b73083cfc2_concultation%20cocnluded.png").unwrap(),
                Url::from_str(&(BLOG_BASE_PATH.to_owned() + "token-holder-consultation-result")).unwrap()
            ),
            BlogPost::new(
                "Token Holder Consultation Is Now Live".to_owned(),
                Url::from_str("https://cdn.prod.website-files.com/60540b6d30224a7cb015203a/681dcf8365388c9dbe4d10c0_Token%20Holder%20Consultation%20request%20for%20input.png").unwrap(),
                Url::from_str(&(BLOG_BASE_PATH.to_owned() + "token-holder-consultation-is-now-live")).unwrap()
            )
        ]
        )
    }
}
