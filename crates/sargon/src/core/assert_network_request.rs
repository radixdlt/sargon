use crate::prelude::*;

/// Verifies that the given `NetworkRequest` has the expected body.
#[cfg(not(tarpaulin_include))]
pub fn assert_network_request<T>(result: NetworkRequest, expected: &T)
where
    T: ?Sized + Serialize,
{
    let expected_body =
        serde_json::to_vec(&expected).unwrap();

    assert_eq!(result.body.bytes, expected_body);
}
