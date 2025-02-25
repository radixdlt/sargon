use crate::prelude::*;

impl HasSampleValues for SubintentHash {
    fn sample() -> Self {
        SubintentHash::new(Hash::sample(), NetworkID::Mainnet)
    }

    fn sample_other() -> Self {
        SubintentHash::new(Hash::sample_other(), NetworkID::Simulator)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SubintentHash;

    #[test]
    fn from_hash() {
        let hash: Hash =
            "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad"
                .parse()
                .unwrap();
        pretty_assertions::assert_eq!(
            SUT::new(hash, NetworkID::Simulator),
            SUT::sample_other()
        )
    }

    #[test]
    fn into_hash() {
        pretty_assertions::assert_eq!(
            Hash::from(SUT::sample_other()),
            "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad"
                .parse::<Hash>()
                .unwrap()
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample()),
            SUT::sample().bech32_encoded_tx_id.to_string()
        );
        pretty_assertions::assert_eq!(format!("{}", SUT::sample()), "subtxid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sy6hgte");
        pretty_assertions::assert_eq!(format!("{}", SUT::sample_other()), "subtxid_sim1v7wlh0dpd5lev6w6s4f2kev562cygmgrm9kqw6swe8w92r4yr7ksuk9pw5");
    }

    #[test]
    fn debug() {
        pretty_assertions::assert_eq!(format!("{:?}", SUT::sample()), "subtxid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sy6hgte");
    }
}
