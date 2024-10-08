use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ChildIntents {
    pub children: Vec<ChildSubintent>,
}

impl From<ChildIntents> for Vec<ScryptoChildSubintent> {
    fn from(value: ChildIntents) -> Self {
        value.children.into_iter().map(Into::into).collect()
    }
}

impl From<(Vec<ScryptoChildSubintent>, NetworkID)> for ChildIntents {
    fn from(value: (Vec<ScryptoChildSubintent>, NetworkID)) -> Self {
        Self {
            children: value
                .0
                .into_iter()
                .map(|c| (c, value.1).into())
                .collect(),
        }
    }
}

impl ChildIntents {
    pub(crate) fn empty() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl HasSampleValues for ChildIntents {
    fn sample() -> Self {
        Self {
            children: vec![ChildSubintent::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            children: vec![
                ChildSubintent::sample(),
                ChildSubintent::sample_other(),
            ],
        }
    }
}
