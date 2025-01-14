use crate::prelude::*;

impl ArculusWalletClient {
    pub async fn sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>> {
        self.execute_card_operation(|wallet| {
            self._sign_hashes(wallet, factor_source, pin, hashes)
        })
        .await
    }
}

impl ArculusWalletClient {
    async fn _sign_hashes(
        &self,
        wallet: ArculusWalletPointer,
        factor_source: ArculusCardFactorSource,
        pin: String,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>> {
        self.validate_factor_source(wallet, factor_source).await?;
        self.verify_pin_io(wallet, pin.clone()).await?;

        let mut per_hash_signatures = IndexMap::new();

        for (hash, paths) in hashes {
            for path in paths {
                let signature = self
                    .sign_hash_path(wallet.clone(), hash.clone(), path)
                    .await?;
                per_hash_signatures
                    .append_or_insert_element_to(hash, signature);
            }
        }

        Ok(per_hash_signatures)
    }

    async fn sign_hash_path(
        &self,
        wallet: ArculusWalletPointer,
        hash: Hash,
        path: DerivationPath,
    ) -> Result<SignatureWithPublicKey> {
        let signature = self
            .sign_hash_path_io(
                wallet,
                path.to_hd_path().clone(),
                hash,
                CardCurve::Ed25519Curve,
                CardAlgorithm::Eddsa,
            )
            .await?;
        let public_key = self._derive_public_key(wallet, path).await?;

        Ok(SignatureWithPublicKey::Ed25519 {
            public_key: public_key,
            signature: signature,
        })
    }
}

impl ArculusWalletClient {
    /// Signs the hash by path on the card
    pub(crate) async fn sign_hash_path_io(
        &self,
        wallet: ArculusWalletPointer,
        path: HDPath,
        hash: Hash,
        curve: CardCurve,
        algorithm: CardAlgorithm,
    ) -> Result<Ed25519Signature> {
        let signature_bytes = self
            .do_chainned_card_io(
                self.csdk_driver.sign_hash_path_request(
                    wallet,
                    path.to_string().into_bytes().into(),
                    curve.val(),
                    algorithm.val(),
                    hash.bytes().into(),
                )?,
                |response| {
                    self.csdk_driver.sign_hash_path_response(wallet, response)
                },
            )
            .await?;

        Ed25519Signature::try_from(signature_bytes)
    }
}
