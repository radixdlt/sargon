import SargonUniFFI

extension Address {
	public func formatted(_ format: AddressFormat) -> String {
		addressFormatted(address: self, format: format)
	}

	public init(validatingAddress bech32String: String) throws {
		self = try newAddressFromBech32(string: bech32String)
	}
	
	public var networkID: NetworkID {
		addressNetworkId(address: self)
	}
	
	public var address: String {
		addressToString(address: self)
	}
}

#if DEBUG
extension Address {
	public func mapTo(networkID: NetworkID) -> Address {
		addressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif