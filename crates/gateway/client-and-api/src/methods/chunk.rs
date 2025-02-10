use crate::prelude::*;
use std::future::Future;

impl GatewayClient {
    pub async fn batch_fetch_chunking<In0, In1, Out1, Out2, F, Fut>(
        &self,
        in_items: impl IntoIterator<Item = In0>,
        build_request: impl Fn(Vec<In0>) -> In1,
        make_request: F,
        aggregate: impl Fn(Vec<Out1>) -> Result<Out2>,
    ) -> Result<Out2>
    where
        In0: Clone,
        F: Fn(In1) -> Fut,
        Fut: Future<Output = Result<Out1>>,
    {
        let in_items = in_items.into_iter().collect_vec();

        let chunks = in_items
            .chunks(GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES as usize)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<_>>>();

        let requests =
            chunks.into_iter().map(build_request).collect::<Vec<_>>();

        let responses =
            futures::future::join_all(requests.into_iter().map(make_request))
                .await
                .into_iter()
                .collect::<Result<Vec<Out1>>>()?;

        aggregate(responses)
    }
}
