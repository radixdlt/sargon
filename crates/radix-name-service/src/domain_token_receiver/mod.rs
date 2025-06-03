use crate::prelude::*;

impl RadixNameService {
    pub(crate) async fn _resolve_receiver_account_for_domain(
        &self,
        domain: RnsDomain,
    ) -> Result<RnsDomainConfiguredReceiver> {
        let domain_details = self.fetch_domain_details(domain.clone()).await?;
        if domain != domain_details.domain {
            return Err(CommonError::RnsInvalidDomainConfiguration {
                reason: "Domain details: domain mismatch".to_owned(),
            });
        }

        self.check_domain_authenticity(domain_details.clone())
            .await?;

        let record = self
            .resolve_record(domain.clone(), Docket::wildcard_receiver())
            .await?;

        let account = match record.value {
            ProgrammaticScryptoSborValue::String(account_str) => {
                AccountAddress::from_str(&account_str.value)?
            }
            _ => {
                return Err(CommonError::RnsInvalidDomainConfiguration {
                    reason: "Configured receiver is not an account address"
                        .to_owned(),
                })
            }
        };

        Ok(RnsDomainConfiguredReceiver::new(domain_details, account))
    }
}

/// Represents a configured receiver for a domain in the Radix Name Service (RNS).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RnsDomainConfiguredReceiver {
    /// The owning domain details, useful for in wallet display.
    pub domain: RnsDomainDetails,
    /// The account address that is configured to receive tokens for the domain.
    pub receiver: AccountAddress,
}

impl HasSampleValues for RnsDomainConfiguredReceiver {
    fn sample() -> Self {
        RnsDomainConfiguredReceiver::new(
            RnsDomainDetails::sample(),
            AccountAddress::sample_mainnet(),
        )
    }

    fn sample_other() -> Self {
        RnsDomainConfiguredReceiver::new(
            RnsDomainDetails::sample_other(),
            AccountAddress::sample_mainnet_other(),
        )
    }
}

impl RnsDomainConfiguredReceiver {
    pub fn new(domain: RnsDomainDetails, receiver: AccountAddress) -> Self {
        Self { domain, receiver }
    }
}

#[cfg(test)]
mod pub_api_tests {
    use super::*;
    use prelude::fixture_gw_model;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixNameService;

    #[actix_rt::test]
    async fn test_resolve_receiver_account_for_domain() {
        let (_, json) = fixture_and_json::<StateNonFungibleDataResponse>(
            fixture_gw_model!("state/request_non_fungible_data_domain_record"),
        )
        .unwrap();

        let body = json.serialize_to_bytes().unwrap();

        let mock_antenna =
            MockNetworkingDriver::new(200, body);

        let sut =
            SUT::new_xrd_domains(Arc::new(mock_antenna), NetworkID::Mainnet)
                .unwrap();

        let domain = RnsDomain::new("grenadine.xrd".to_owned());
        let result = sut
            .resolve_receiver_account_for_domain(domain.clone())
            .await
            .unwrap();

        let expected_domain_details = RnsDomainDetails::new(
                domain,
                AccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap(),
                "#FF5722".to_owned(),
                "#D32F2F".to_owned(),
            );

        assert_eq!(
            result,
            RnsDomainConfiguredReceiver::new(
                expected_domain_details,
                AccountAddress::from_str("account_rdx128pu3gp74hgl0a9d6d899vd0nn8wh5z0syrkvp28hd492dk0u8fe8t").unwrap()
            )
        );
    }
}
