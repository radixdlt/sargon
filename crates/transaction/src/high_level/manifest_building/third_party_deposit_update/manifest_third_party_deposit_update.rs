use crate::prelude::*;

use radix_engine_interface::blueprints::account::{
    ACCOUNT_ADD_AUTHORIZED_DEPOSITOR, ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
    ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
    ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
};

pub trait IsThirdPartyDepositsUpdating: Sized {
    fn third_party_deposit_update_by_delta(
        owner: &AccountAddress,
        delta: ThirdPartyDepositsDelta,
    ) -> Self;

    fn third_party_deposit_update(
        owner: &AccountAddress,
        from: ThirdPartyDeposits,
        to: ThirdPartyDeposits,
    ) -> Self {
        let delta = ThirdPartyDepositsDelta::new(from, to);
        Self::third_party_deposit_update_by_delta(owner, delta)
    }
}

impl IsThirdPartyDepositsUpdating for TransactionManifest {
    fn third_party_deposit_update_by_delta(
        owner: &AccountAddress,
        delta: ThirdPartyDepositsDelta,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();

        if let Some(deposit_rule) = delta.deposit_rule {
            builder = builder.call_method(
                owner,
                ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
                (deposit_rule,),
            );
        }

        for resource_address in delta.asset_exceptions_to_be_removed {
            builder = builder.call_method(
                owner,
                ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                (resource_address,),
            )
        }

        for asset_exception in delta.asset_exceptions_to_add_or_update {
            builder = builder.call_method(
                owner,
                ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                asset_exception,
            )
        }

        for depositor_address in delta.depositor_addresses_to_remove {
            builder = builder.call_method(
                owner,
                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR,
                depositor_address,
            )
        }

        for depositor_address in delta.depositor_addresses_to_add {
            builder = builder.call_method(
                owner,
                ACCOUNT_ADD_AUTHORIZED_DEPOSITOR,
                depositor_address,
            )
        }

        TransactionManifest::sargon_built(builder, owner.network_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn update_third_party_deposits() {
        let owner:AccountAddress = "account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf".into();
        let to_json = r#"
        {
            "assetsExceptionList" : [
              {
                "address" : "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
                "exceptionRule" : "allow"
              },
              {
                "address" : "resource_tdx_2_1t4kep9ldg9t0cszj78z6fcr2zvfxfq7muetq7pyvhdtctwxum90scq",
                "exceptionRule" : "allow"
              },
              {
                "address" : "resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3",
                "exceptionRule" : "deny"
              }
            ],
            "depositorsAllowList" : [
              {
                "discriminator" : "resourceAddress",
                "value" : "resource_tdx_2_1ngw6cufaxs5p82kw49juy2yfkt53se76vr0xfsu3tvyduuw6s0y6lc"
              }
            ],
            "depositRule" : "acceptKnown"
          }
        "#;
        let to = serde_json::from_str(to_json).unwrap();
        let manifest = SUT::third_party_deposit_update(
            &owner,
            ThirdPartyDeposits::default(),
            to,
        );
        manifest_eq(
            manifest,
            r#"
            CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "set_default_deposit_rule"
            Enum<2u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "set_resource_preference"
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "set_resource_preference"
            Address("resource_tdx_2_1t4kep9ldg9t0cszj78z6fcr2zvfxfq7muetq7pyvhdtctwxum90scq")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "set_resource_preference"
            Address("resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3")
            Enum<1u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "add_authorized_depositor"
            Enum<1u8>(
                Address("resource_tdx_2_1ngw6cufaxs5p82kw49juy2yfkt53se76vr0xfsu3tvyduuw6s0y6lc")
            )
        ;
    "#,
        )
    }
}
