use crate::prelude::*;
use std::future::Future;

impl GatewayClient {
    /// Load all pages of a paginated API call that returns a `PageResponse`.
    ///
    /// Returns: A collection of the items from all pages.
    pub async fn load_all_pages<T, F, Fut>(
        &self,
        account_address: AccountAddress,
        api_call: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(AccountPageRequest) -> Fut,
        Fut: Future<Output = Result<PageResponse<T>>>,
    {
        let mut items: Vec<T> = Vec::new();
        let mut more_to_load = true;
        let mut cursor: Option<String> = None;
        while more_to_load {
            let request = AccountPageRequest::new(
                account_address,
                cursor.clone(),
                GATEWAY_PAGE_REQUEST_LIMIT,
            );
            let response = api_call(request).await?;
            items.extend(response.items);
            cursor = response.next_cursor;
            more_to_load = cursor.is_some();
        }

        Ok(items)
    }
}

#[cfg(test)]
mod load_all_pages_tests {
    use crate::prelude::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    #[actix_rt::test]
    async fn one_page_only() {
        // Test the case where we load only one page with two elements.
        let item_one = AccountResourcePreference::sample();
        let item_two = AccountResourcePreference::sample_other();
        let response =
            mock_page_response(None, vec![item_one.clone(), item_two.clone()]);

        let mock_driver =
            MockNetworkingDriver::new_with_responses(vec![response]);
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        let result = sut
            .load_all_pages(AccountAddress::sample(), |request| {
                sut.account_resource_preferences(request)
            })
            .await
            .unwrap();

        assert_eq!(result, vec![item_one, item_two]);
    }

    #[actix_rt::test]
    async fn two_pages() {
        // Test the case where we load two pages with one element each
        let item_one = AccountResourcePreference::sample();
        let response_one = mock_page_response(
            "cursor_one".to_string(),
            vec![item_one.clone()],
        );
        let item_two = AccountResourcePreference::sample_other();
        let response_two = mock_page_response(None, vec![item_two.clone()]);

        let mock_driver = MockNetworkingDriver::new_with_responses_and_spy(
            vec![response_one, response_two],
            |request, count| {
                // Verify the correct body is sent on each request
                let cursor = if count == 1 {
                    Some("cursor_one".to_string())
                } else {
                    None
                };
                // Verify the body sent matches the expected one
                let expected_request = AccountPageRequest::new(
                    AccountAddress::sample(),
                    cursor,
                    GATEWAY_PAGE_REQUEST_LIMIT,
                );
                let expected_body =
                    serde_json::to_vec(&expected_request).unwrap();

                assert_eq!(request.body.bytes, expected_body);
            },
        );
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        let result = sut
            .load_all_pages(AccountAddress::sample(), |request| {
                sut.account_resource_preferences(request)
            })
            .await
            .unwrap();

        assert_eq!(result, vec![item_one, item_two]);
    }

    #[actix_rt::test]
    async fn failure() {
        // Test the case where the first page fails.
        let mock_driver = MockNetworkingDriver::new_always_failing();
        let sut = SUT::with_gateway(Arc::new(mock_driver), Gateway::stokenet());

        let result = sut
            .load_all_pages(AccountAddress::sample(), |request| {
                sut.account_resource_preferences(request)
            })
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::NetworkResponseBadCode);
    }

    /// Creates a `MockNetworkingDriverResponse` for a `PageResponse`.
    fn mock_page_response(
        cursor: impl Into<Option<String>>,
        items: Vec<AccountResourcePreference>,
    ) -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(PageResponse::new(
            1, cursor, items,
        ))
    }
}