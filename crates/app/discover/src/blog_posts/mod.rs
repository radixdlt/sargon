pub const CACHE_BLOG_POSTS_PATH: &str = "blog_posts.json";
pub const BLOG_BASE_PATH: &str = "https://www.radixdlt.com/blog/";
pub const BLOG_POSTS_URL: &str = "https://webflow-blog-posts-proxy.radixdlt.com/v2/collections/649aa8a9681ec6168a57d972/items/live?offset=0&limit=20";
pub const BLOG_POSTS_DETAILS_URL: &str = "https://webflow-blog-posts-proxy.radixdlt.com/v2/collections/649aa8a9681ec6168a57d972";

mod blog_post;
mod client;
mod client_caching;
mod client_remote_fetch;

pub use blog_post::*;
pub use client::*;
