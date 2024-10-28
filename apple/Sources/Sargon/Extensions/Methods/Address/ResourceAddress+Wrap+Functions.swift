import SargonUniFFI

extension ResourceAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newResourceAddress(bech32: bech32String)
	}

	public var networkID: NetworkId {
        self.networkId
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }

	/// If this is an address of a **fungible** resource or not.
	public var isFungible: Bool {
        self.kind == .fungible
	}

	/// If this is an address of a **non-fungible** resource or not.
	public var isNonFungible: Bool {
        self.kind == .nonFungible
	}
	
	/// Returns the XRD resource on network identified by `networkID`.
	public static func xrd(on networkID: NetworkID) -> Self {
		xrdAddressOfNetwork(networkId: networkID)
	}
}

#if DEBUG
extension ResourceAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newResourceAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		resourceAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
