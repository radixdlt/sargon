use crate::prelude::*;

pub trait OnSameNetworkValidating:
    Clone + IntoIterator<Item = Self::Element>
{
    type Element: IsNetworkAware;

    fn is_empty(&self) -> bool;

    fn assert_elements_not_empty_and_on_same_network(
        &self,
    ) -> Result<NetworkID> {
        self.assert_elements_on_same_network()
            .and_then(|x| x.ok_or(CommonError::ExpectedNonEmptyCollection))
    }

    fn assert_elements_on_same_network(&self) -> Result<Option<NetworkID>>;
}

impl<T: IsNetworkAware, U: Clone + IntoIterator<Item = T>>
    OnSameNetworkValidating for U
{
    type Element = T;

    fn is_empty(&self) -> bool {
        self.clone().into_iter().next().is_none()
    }

    fn assert_elements_on_same_network(&self) -> Result<Option<NetworkID>> {
        if self.is_empty() {
            return Ok(None);
        }
        let network_id = self.clone().into_iter().next().unwrap().network_id();
        self.clone().into_iter().try_for_each(|e| {
            if e.network_id() == network_id {
                Ok(())
            } else {
                Err(CommonError::NetworkDiscrepancy {
                    expected: network_id,
                    actual: e.network_id(),
                })
            }
        })?;

        Ok(Some(network_id))
    }
}
