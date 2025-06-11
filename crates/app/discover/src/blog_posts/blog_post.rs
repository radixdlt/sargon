use crate::prelude::*;

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
