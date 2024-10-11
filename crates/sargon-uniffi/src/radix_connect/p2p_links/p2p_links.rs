use crate::prelude::*;

decl_identified_vec_of!(
    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with the dApp or Connector Extension
    P2PLink
);

json_data_convertible!(P2PLinks);
