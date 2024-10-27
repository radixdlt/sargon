import SargonUniFFI

extension AccessControllerAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newAccessControllerAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }

	public var networkID: NetworkId {
        self.networkId
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
