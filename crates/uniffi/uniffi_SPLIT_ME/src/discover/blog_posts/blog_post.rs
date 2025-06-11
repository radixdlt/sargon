use crate::prelude::*;
use sargon::BlogPost as InternalBlogPost;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct BlogPost {
    pub name: String,
    pub image: Url,
    pub url: Url,
}
