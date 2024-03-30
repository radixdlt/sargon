import SargonUniFFI

extension PackageAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newPackageAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        packageAddressFormatted(address: self, format: format)
    }

	/// The bech32 encoded string for this address.
	public var address: String {
		packageAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		packageAddressNetworkId(address: self)
	}
	
	public func embed() -> Address {
		.package(self)
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
