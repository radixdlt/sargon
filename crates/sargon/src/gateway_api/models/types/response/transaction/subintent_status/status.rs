use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum SubintentStatus {
    Unknown,
    CommittedSuccess,
}
