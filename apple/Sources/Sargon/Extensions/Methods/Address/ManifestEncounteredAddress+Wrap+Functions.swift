import SargonUniFFI

extension ManifestEncounteredAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newManifestEncounteredAddressFromBech32(string: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		manifestEncounteredAddressToString(address: self)
	}

	public var networkID: NetworkId {
		manifestEncounteredAddressNetworkId(address: self)
	}
	
	public func formatted(_ format: AddressFormat) -> String {
		manifestEncounteredAddressFormatted(address: self, format: format)
	}
	

}

#if DEBUG
extension ManifestEncounteredAddress {
	public static func random(networkID: NetworkID) -> Self {
		.component(newComponentAddressRandom(networkId: networkID))
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		manifestEncounteredAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
