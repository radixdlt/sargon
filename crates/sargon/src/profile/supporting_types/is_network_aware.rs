use crate::prelude::*;

pub trait IsNetworkAware {
    fn network_id(&self) -> NetworkID;

    /// Validates that `other` is on the same network as self.
    fn is_on_same_network_as(&self, other: &impl IsNetworkAware) -> Result<()> {
        todo!()
        // let this = self.network_id();
        // let other = other.network_id();
        // if this != other {
        //     Err(CommonError::NetworkDiscrepancy {
        //         expected: this,
        //         actual: other,
        //     })
        // } else {
        //     Ok(())
        // }
    }
}
