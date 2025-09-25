import SargonUniFFI

// MARK: - TransactionManifest + SargonModel
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

	public static func accountLockerClaim(
		lockerAddress: LockerAddress,
		claimant: AccountAddress,
		claimableResources: [AccountLockerClaimableResource]
	) -> Self {
		manifestAccountLockerClaim(
			lockerAddress: lockerAddress,
			claimant: claimant,
			claimableResources: claimableResources
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
		addressOfOwner: AccountAddress,
		count: UInt8? = 10
	) -> Self {
		manifestCreateMultipleFungibleTokens(
			addressOfOwner: addressOfOwner,
			count: count
		)
	}

	public static func createMultipleNonFungibleTokens(
		addressOfOwner: AccountAddress,
		collectionCount: UInt8? = nil,
		nftsPerCollection: UInt8? = nil
	) -> Self {
		manifestCreateMultipleNonFungibleTokens(
			addressOfOwner: addressOfOwner,
			collectionCount: collectionCount,
			nftsPerCollection: nftsPerCollection
		)
	}

	public static func createNonFungibleToken(
		addressOfOwner: AccountAddress,
		nftsPerCollection: UInt8? = nil
	) -> Self {
		manifestCreateNonFungibleToken(
			addressOfOwner: addressOfOwner,
			nftsPerCollection: nftsPerCollection
		)
	}
}
#endif // DEBUG
