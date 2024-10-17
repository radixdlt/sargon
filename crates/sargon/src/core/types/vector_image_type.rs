use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorImageType {
    Svg,
    Pdf,
}

impl VectorImageType {
    pub fn url_extension(&self) -> &str {
        match self {
            VectorImageType::Svg => ".svg",
            VectorImageType::Pdf => ".pdf",
        }
    }

    pub fn data_url_type(&self) -> &str {
        match self {
            VectorImageType::Svg => "svg+xml",
            VectorImageType::Pdf => "pdf",
        }
    }
}

impl HasSampleValues for VectorImageType {
    fn sample() -> Self {
        Self::Svg
    }

    fn sample_other() -> Self {
        Self::Pdf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VectorImageType;

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn test_url_extension() {
        assert_eq!(SUT::sample().url_extension(), ".svg");
        assert_eq!(SUT::sample_other().url_extension(), ".pdf");
    }

    #[test]
    fn test_data_url_type() {
        assert_eq!(SUT::sample().data_url_type(), "svg+xml");
        assert_eq!(SUT::sample_other().data_url_type(), "pdf");
    }
}
