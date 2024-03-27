extension ComponentAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newComponentAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        componentAddressFormatted(address: self, format: format)
    }

	/// The bech32 encoded string for this address.
	public var address: String {
		componentAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		componentAddressNetworkId(address: self)
	}
}

#if DEBUG
extension ComponentAddress {
	public func embed() -> Address {
		.component(self)
	}
	public func mapTo(networkID: NetworkID) -> Self {
		componentAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
