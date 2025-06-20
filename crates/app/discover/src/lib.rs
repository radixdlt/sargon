mod blog_posts;

pub mod prelude {
    pub use crate::blog_posts::*;

    pub(crate) use clients::prelude::*;
    pub use prelude::prelude::*;
    pub(crate) use serde::{Deserialize, Serialize};
}
