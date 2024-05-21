mod abstract_entity_type;
mod appearance_id;
mod appearance_id_uniffi_fn;
mod bag_of_bytes;
mod bag_of_bytes_uniffi_fn;
mod decimal192;
mod decimal192_uniffi_fn;
mod entity_kind;
mod epoch;
mod exactly_n_bytes;
mod identifiable;
mod keys;
mod locale_config;
mod logged_result;
mod network_id;
mod network_id_uniffi_fn;
mod non_empty_max_n_bytes;
mod nonce;
mod nonce_uniffi_fn;
mod requested_number_quantifier;
mod requested_quantity;
mod requested_quantity_uniffi_fn;
mod rounding_mode;
mod safe_to_log;
mod secret_bytes;
mod signatures;

pub use abstract_entity_type::*;
pub use appearance_id::*;
pub use appearance_id_uniffi_fn::*;
pub use bag_of_bytes::*;
pub use bag_of_bytes_uniffi_fn::*;
pub use decimal192::*;
pub use decimal192_uniffi_fn::*;
pub use entity_kind::*;
pub use epoch::*;
pub use exactly_n_bytes::*;
pub use identifiable::*;
pub use keys::*;
pub use locale_config::*;
pub use logged_result::*;
pub use network_id::*;
pub use network_id_uniffi_fn::*;
pub use non_empty_max_n_bytes::*;
pub use nonce::*;
pub use nonce_uniffi_fn::*;
pub use requested_number_quantifier::*;
pub use requested_quantity::*;
pub use requested_quantity_uniffi_fn::*;
pub use rounding_mode::*;
pub use safe_to_log::*;
pub use secret_bytes::*;
pub use signatures::*;

impl From<Exactly32Bytes> for aes_gcm::Key<aes_gcm::Aes256Gcm> {
    fn from(value: Exactly32Bytes) -> Self {
        Self::from(*value.bytes())
    }
}
