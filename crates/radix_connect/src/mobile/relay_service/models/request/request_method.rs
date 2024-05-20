use crate::prelude::*;

// Ref API docs at https://github.com/radixdlt/radix-connect-relay?tab=readme-ov-file#api-v1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    SendRequest,
    GetRequests,
    SendResponse,
    GetResponses,
    SendHandshakeRequest,
    GetHandshakeRequest,
    SendHandshakeResponse,
    GetHandshakeResponse,
}

impl HasSampleValues for Method {
    fn sample() -> Self {
        Method::SendRequest
    }

    fn sample_other() -> Self {
        Method::GetRequests
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Method;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
