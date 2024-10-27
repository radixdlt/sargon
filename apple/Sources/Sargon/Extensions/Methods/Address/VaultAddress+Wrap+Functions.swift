import SargonUniFFI

extension VaultAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newVaultAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }

	public var networkID: NetworkId {
        self.networkId
	}

	/// If this is an address of a **fungible** vault or not.
	public var isFungible: Bool {
        self.kind == .fungible
	}

	/// If this is an address of a **non-fungible** vault or not.
	public var isNonFungible: Bool {
        self.kind == .nonFungible
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
