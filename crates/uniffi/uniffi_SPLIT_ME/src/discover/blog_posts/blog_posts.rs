use crate::prelude::*;
use sargon::BlogPost as InternalBlogPost;
use sargon::BlogPosts as InternalBlogPosts;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct BlogPosts {
    /// All of the blog posts
    pub posts: Vec<BlogPost>,
    /// The most recent new blog post.
    /// There might be more than one new blog post, but we'll inform the user
    /// about the latest one only.
    pub new_blog_post: Option<BlogPost>,
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct BlogPost {
    pub name: String,
    pub image: Url,
    pub url: Url,
}
