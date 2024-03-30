import SargonUniFFI

extension VaultAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newVaultAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        vaultAddressFormatted(address: self, format: format)
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
	
	public func embed() -> Address {
		.vault(self)
	}
}

#if DEBUG
extension VaultAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newVaultAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		vaultAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
