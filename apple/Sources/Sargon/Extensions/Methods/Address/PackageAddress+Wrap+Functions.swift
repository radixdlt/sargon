import SargonUniFFI

extension PackageAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newPackageAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }

	public var networkID: NetworkId {
        self.networkId
	}

}

#if DEBUG
extension PackageAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newPackageAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		packageAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
