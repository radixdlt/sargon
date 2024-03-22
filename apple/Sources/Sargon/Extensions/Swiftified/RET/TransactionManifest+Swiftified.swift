extension TransactionManifest: @unchecked Sendable {}
extension TransactionManifest: SargonModel {}

// MARK: Build Manifest
extension TransactionManifest {
    
    // Not DEBUG only, since used in PROD for Stokenet.
    public static func faucet(
        includeLockFeeInstruction: Bool,
        addressOfReceivingAccount: AccountAddress
    ) -> Self {
        manifestForFaucet(
            includeLockFeeInstruction: includeLockFeeInstruction,
            addressOfReceivingAccount: addressOfReceivingAccount
        )
    }
    
    public static func setOwnerKeys(
        addressOfAccountOrPersona: AddressOfAccountOrPersona,
        ownerKeyHashes: [PublicKeyHash]
    ) -> Self {
        manifestSetOwnerKeysHashes(
            addressOfAccountOrPersona: addressOfAccountOrPersona,
            ownerKeyHashes: ownerKeyHashes
        )
    }
    
    public static func stakesClaim(
        accountAddress: AccountAddress,
        stakeClaims: [StakeClaim]
    ) -> Self {
        manifestStakesClaim(
            accountAddress: accountAddress,
            stakeClaims: stakeClaims
        )
    }
    
    public static func assetsTransfers(
        transfers: PerAssetTransfers
    ) -> Self {
       manifestPerAssetTransfers(transfers: transfers)
    }

    public static func markingAccountAsDappDefinitionType(
        accountAddress: AccountAddress
    ) -> Self {
        manifestMarkingAccountAsDappDefinitionType(accountAddress: accountAddress)
    }
    
    public static func thirdPartyDepositUpdate(
        accountAddress: AccountAddress,
        from currentInProfile: ThirdPartyDeposits,
        to newFromUI: ThirdPartyDeposits
    ) -> Self {
        manifestThirdPartyDepositUpdate(
            accountAddress: accountAddress,
            from: currentInProfile,
            to: newFromUI
        )
    }
}

// MARK: Modify Manifest
extension TransactionManifest {
    public func modify(
        lockFee fee: Decimal192 = .temporaryStandardFee,
        addressOfFeePayer: AccountAddress
    ) -> Self {
        modifyManifestLockFee(
            manifest: self,
            addressOfFeePayer: addressOfFeePayer,
            fee: fee
        )
    }

    public func modify(
        addGuarantees guarantees: [TransactionGuarantee]
    ) -> Self {
        modifyManifestAddGuarantees(
            manifest: self,
            guarantees: guarantees
        )
    }
}

#if DEBUG
extension TransactionManifest {

    public static func createFungibleToken(
        addressOfOwner: AccountAddress
    ) -> Self {
        manifestCreateFungibleToken(addressOfOwner: addressOfOwner)
    }
    
    public static func createFungibleTokenWithMetadata(
        addressOfOwner: AccountAddress,
        initialSupply: Decimal192,
        metadata: TokenDefinitionMetadata
    ) -> Self {
        manifestCreateFungibleTokenWithMetadata(
            addressOfOwner: addressOfOwner,
            initialSupply: initialSupply,
            metadata: metadata
        )
    }
    
    
    public static func createMultipleFungibleTokens(
        addressOfOwner: AccountAddress
    ) -> Self {
       manifestCreateMultipleFungibleTokens(addressOfOwner: addressOfOwner)
    }

    public static func createMultipleNonFungibleTokens(
        addressOfOwner: AccountAddress
    ) -> Self {
       manifestCreateMultipleNonFungibleTokens(addressOfOwner: addressOfOwner)
    }

    public static func createNonFungibleToken(
        addressOfOwner: AccountAddress
    ) -> Self {
        manifestCreateNonFungibleToken(addressOfOwner: addressOfOwner)
    }
    
}
#endif // DEBUG
