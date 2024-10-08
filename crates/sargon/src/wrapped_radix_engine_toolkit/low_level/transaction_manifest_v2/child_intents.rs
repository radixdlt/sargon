use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ChildIntents {
    pub children: Vec<ChildSubintent>, // secret_magic?
    pub network_id: NetworkID,         // do we need network_id here?
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
            network_id: value.1,
        }
    }
}

impl ChildIntents {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            children: Vec::new(),
            network_id,
        }
    }
}

impl HasSampleValues for ChildIntents {
    fn sample() -> Self {
        Self {
            children: vec![ChildSubintent::sample()],
            network_id: NetworkID::Mainnet,
        }
    }

    fn sample_other() -> Self {
        Self {
            children: vec![
                ChildSubintent::sample(),
                ChildSubintent::sample_other(),
            ],
            network_id: NetworkID::Simulator,
        }
    }
}
