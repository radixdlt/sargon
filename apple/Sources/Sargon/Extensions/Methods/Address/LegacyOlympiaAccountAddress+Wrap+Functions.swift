import SargonUniFFI

extension LegacyOlympiaAccountAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newLegacyOlympiaAccountAddressFromString(
			string: bech32String
		)
	}

	public init(publicKey: Secp256k1PublicKey) {
		self = newLegacyOlympiaAccountAddressFromPublicKey(
			publicKey: publicKey
		)
	}

	public func formatted(_ format: AddressFormat = .default) -> String {
		legacyOlympiaAccountAddressFormatted(address: self, format: format)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		legacyOlympiaAccountAddressToString(
			address: self
		)
	}

	public var networkID: NetworkId {
		// We do not allow creation of Non-Mainnet Olympia Addresses.
		.mainnet
	}

	public func toBabylonAddress() -> AccountAddress {
		legacyOlympiaAccountAddressToBabylonAccountAddress(
			address: self
		)
	}

	public func isLegacyOfBabylonAddress(_ babylon: AccountAddress) -> Bool {
		legacyOlympiaAccountAddressIsLegacyOfBabylon(
			legacyOlympiaAddress: self,
			babylonAccountAddress: babylon
		)
	}
}

extension AccountAddress {
	public func wasMigratedFromLegacyOlympia(
		address legacy: LegacyOlympiaAccountAddress
	) -> Bool {
		legacy.isLegacyOfBabylonAddress(self)
	}
}
