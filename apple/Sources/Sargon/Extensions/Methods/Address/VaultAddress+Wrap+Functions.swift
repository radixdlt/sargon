extension VaultAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newVaultAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		vaultAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		vaultAddressNetworkId(address: self)
	}

	/// If this is an address of a **fungible** vault or not.
	public var isFungible: Bool {
		vaultAddressIsFungible(address: self)
	}

	/// If this is an address of a **non-fungible** vault or not.
	public var isNonFungible: Bool {
		vaultAddressIsNonFungible(address: self)
	}
}
