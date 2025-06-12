use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlogPosts {
    /// All of the blog posts
    pub posts: Vec<BlogPost>,
    /// The most recent new blog post.
    /// There might be more than one new blog post, but we'll inform the user
    /// about the latest one only.
    pub new_blog_post: Option<BlogPost>,
}

impl BlogPosts {
    pub fn new(posts: Vec<BlogPost>, new_blog_post: Option<BlogPost>) -> Self {
        Self {
            posts,
            new_blog_post,
        }
    }
}

impl HasSampleValues for BlogPosts {
    fn sample() -> Self {
        Self::new(vec![BlogPost::sample()], Some(BlogPost::sample_other()))
    }

    fn sample_other() -> Self {
        Self::new(vec![BlogPost::sample_other()], Some(BlogPost::sample()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlogPost {
    pub name: String,
    pub image: Url,
    pub url: Url,
}

impl BlogPost {
    pub fn new(name: String, image: Url, url: Url) -> Self {
        Self { name, image, url }
    }
}

impl HasSampleValues for BlogPost {
    fn sample() -> Self {
        Self::new("blog1".to_owned(), Url::sample(), Url::sample_other())
    }

    fn sample_other() -> Self {
        Self::new("blog2".to_owned(), Url::sample_other(), Url::sample())
    }
}
