import SargonUniFFI

extension ManifestEncounteredComponentAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newManifestEncounteredComponentAddressFromBech32(string: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		manifestEncounteredComponentAddressToString(address: self)
	}

	public var networkID: NetworkId {
		manifestEncounteredComponentAddressNetworkId(address: self)
	}
	
	public func formatted(_ format: AddressFormat) -> String {
		manifestEncounteredComponentAddressFormatted(address: self, format: format)
	}
	

}

#if DEBUG
extension ManifestEncounteredComponentAddress {
	public static func random(networkID: NetworkID) -> Self {
		.component(newComponentAddressRandom(networkId: networkID))
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		manifestEncounteredComponentAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
