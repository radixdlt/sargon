use super::request_method::Method;
use crate::prelude::*;
use crate::radix_connect::mobile::session::session_id::SessionID;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub method: Method,
    pub session_id: SessionID,
    pub data: Option<BagOfBytes>,
}

impl Request {
    pub fn new(
        method: Method,
        session_id: impl Into<SessionID>,
        data: impl Into<Option<BagOfBytes>>,
    ) -> Self {
        Self {
            method,
            session_id: session_id.into(),
            data: data.into(),
        }
    }
}

impl Request {
    pub fn new_send_request(
        session_id: impl Into<SessionID>,
        data: impl Into<BagOfBytes>,
    ) -> Self {
        Self::new(Method::SendRequest, session_id, data.into())
    }

    pub fn new_get_requests(session_id: impl Into<SessionID>) -> Self {
        Self::new(Method::GetRequests, session_id, None)
    }

    pub fn new_send_response(
        session_id: impl Into<SessionID>,
        data: impl Into<BagOfBytes>,
    ) -> Self {
        Self::new(Method::SendResponse, session_id, data.into())
    }

    pub fn new_get_responses(session_id: impl Into<SessionID>) -> Self {
        Self::new(Method::GetResponses, session_id, None)
    }

    pub fn new_get_handshake_request(session_id: impl Into<SessionID>) -> Self {
        Self::new(Method::GetHandshakeRequest, session_id, None)
    }

    pub fn new_send_handshake_response(
        session_id: impl Into<SessionID>,
        data: impl Into<BagOfBytes>,
    ) -> Self {
        Self::new(Method::SendHandshakeResponse, session_id, data.into())
    }

    pub fn new_send_handshake_response_with_public_key(
        session_id: impl Into<SessionID>,
        public_key: impl Into<PublicKey>,
    ) -> Result<Self> {
        let public_key_bytes =
            BagOfBytes::from_hex(public_key.into().to_hex().as_str())?;
        Ok(Self::new_send_handshake_response(
            session_id.into(),
            public_key_bytes,
        ))
    }
}

impl HasSampleValues for Request {
    fn sample() -> Self {
        Self::new_send_request(SessionID::sample(), BagOfBytes::sample())
    }

    fn sample_other() -> Self {
        Self::new_get_requests(SessionID::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Request;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn send_request() {
        let session_id = SessionID::sample();
        let data = BagOfBytes::sample();
        let request = SUT::new_send_request(session_id.clone(), data.clone());
        assert_eq!(request.method, Method::SendRequest);
        assert_eq!(request.session_id, session_id);
        assert_eq!(request.data, Some(data));
    }

    #[test]
    fn get_requests() {
        let session_id = SessionID::sample();
        let request = SUT::new_get_requests(session_id.clone());
        assert_eq!(request.method, Method::GetRequests);
        assert_eq!(request.session_id, session_id);
        assert_eq!(request.data, None);
    }

    #[test]
    fn send_response() {
        let session_id = SessionID::sample();
        let data = BagOfBytes::sample();
        let request = SUT::new_send_response(session_id.clone(), data.clone());
        assert_eq!(request.method, Method::SendResponse);
        assert_eq!(request.session_id, session_id);
        assert_eq!(request.data, Some(data));
    }

    #[test]
    fn get_responses() {
        let session_id = SessionID::sample();
        let request = SUT::new_get_responses(session_id.clone());
        assert_eq!(request.method, Method::GetResponses);
        assert_eq!(request.session_id, session_id);
        assert_eq!(request.data, None);
    }

    #[test]
    fn get_handshake_requests() {
        let session_id = SessionID::sample();
        let request = SUT::new_get_handshake_request(session_id.clone());
        assert_eq!(request.method, Method::GetHandshakeRequest);
        assert_eq!(request.session_id, session_id);
        assert_eq!(request.data, None);
    }

    #[test]
    fn send_handshake_response() {
        let session_id = SessionID::sample();
        let data = BagOfBytes::sample();
        let request =
            SUT::new_send_handshake_response(session_id.clone(), data.clone());
        assert_eq!(request.method, Method::SendHandshakeResponse);
        assert_eq!(request.session_id, session_id);
        assert_eq!(request.data, Some(data));
    }

    #[test]
    fn send_handshake_response_with_public_key() {
        let session_id = SessionID::sample();
        let public_key = PublicKey::sample();
        let request = SUT::new_send_handshake_response_with_public_key(
            session_id.clone(),
            public_key.clone(),
        )
        .unwrap();
        assert_eq!(request.method, Method::SendHandshakeResponse);
        assert_eq!(request.session_id, session_id);
        assert_eq!(
            request.data,
            Some(BagOfBytes::from_hex(public_key.to_hex().as_str()).unwrap())
        );
    }

    #[test]
    fn send_request_json_roundtrip() {
        let session_id = SessionID::sample();
        let data = BagOfBytes::sample();
        let request = SUT::new_send_request(session_id.clone(), data.clone());

        let expected_json = format!(
            r#"
            {{
                    "method":"sendRequest",
                    "sessionId":"{}",
                    "data":"{}"
            }}
            "#,
            session_id, data
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }

    #[test]
    fn get_requests_json_roundtrip() {
        let session_id = SessionID::sample();
        let request = SUT::new_get_requests(session_id.clone());

        let expected_json = format!(
            r#"
            {{
                    "method":"getRequests",
                    "sessionId":"{}"
            }}
            "#,
            session_id
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }

    #[test]
    fn send_response_json_roundtrip() {
        let session_id = SessionID::sample();
        let data = BagOfBytes::sample();
        let request = SUT::new_send_response(session_id.clone(), data.clone());

        let expected_json = format!(
            r#"
            {{
                    "method":"sendResponse",
                    "sessionId":"{}",
                    "data":"{}"
            }}
            "#,
            session_id, data
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }

    #[test]
    fn get_responses_json_roundtrip() {
        let session_id = SessionID::sample();
        let request = SUT::new_get_responses(session_id.clone());

        let expected_json = format!(
            r#"
            {{
                    "method":"getResponses",
                    "sessionId":"{}"
            }}
            "#,
            session_id
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }

    #[test]
    fn get_handshake_request_json_roundtrip() {
        let session_id = SessionID::sample();
        let request = SUT::new_get_handshake_request(session_id.clone());

        let expected_json = format!(
            r#"
            {{
                    "method":"getHandshakeRequest",
                    "sessionId":"{}"
            }}
            "#,
            session_id
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }

    #[test]
    fn send_handshake_response_json_roundtrip() {
        let session_id = SessionID::sample();
        let data = BagOfBytes::sample();
        let request =
            SUT::new_send_handshake_response(session_id.clone(), data.clone());

        let expected_json = format!(
            r#"
            {{
                    "method":"sendHandshakeResponse",
                    "sessionId":"{}",
                    "data":"{}"
            }}
            "#,
            session_id, data
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }

    #[test]
    fn send_handshake_response_with_public_key_json_roundtrip() {
        let session_id = SessionID::sample();
        let public_key = PublicKey::sample();
        let request = SUT::new_send_handshake_response_with_public_key(
            session_id.clone(),
            public_key.clone(),
        )
        .unwrap();

        let expected_json = format!(
            r#"
            {{
                    "method":"sendHandshakeResponse",
                    "sessionId":"{}",
                    "data":"{}"
            }}
            "#,
            session_id,
            BagOfBytes::from_hex(public_key.to_hex().as_str()).unwrap()
        );
        assert_eq_after_json_roundtrip(&request, &expected_json);
    }
}
