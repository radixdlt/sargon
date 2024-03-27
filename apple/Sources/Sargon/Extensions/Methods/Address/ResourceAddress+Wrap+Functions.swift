extension ResourceAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newResourceAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		resourceAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		resourceAddressNetworkId(address: self)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        resourceAddressFormatted(address: self, format: format)
    }

	/// If this is an address of a **fungible** resource or not.
	public var isFungible: Bool {
		resourceAddressIsFungible(address: self)
	}

	/// If this is an address of a **non-fungible** resource or not.
	public var isNonFungible: Bool {
		resourceAddressIsNonFungible(address: self)
	}
	
	/// Returns the XRD resource on network identified by `networkID`.
	public static func xrd(on networkID: NetworkID) -> Self {
		xrdAddressOfNetwork(networkId: networkID)
	}
}

#if DEBUG
extension ResourceAddress {
	public func embed() -> Address {
		.resource(self)
	}
	public func mapTo(networkID: NetworkID) -> Self {
		resourceAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
