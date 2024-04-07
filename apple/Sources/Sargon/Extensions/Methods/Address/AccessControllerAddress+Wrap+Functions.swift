import SargonUniFFI

extension AccessControllerAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newAccessControllerAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        accessControllerAddressFormatted(address: self, format: format)
    }

	/// The bech32 encoded string for this address.
	public var address: String {
		accessControllerAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		accessControllerAddressNetworkId(address: self)
	}
	
}

#if DEBUG
extension AccessControllerAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newAccessControllerAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		accessControllerAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
