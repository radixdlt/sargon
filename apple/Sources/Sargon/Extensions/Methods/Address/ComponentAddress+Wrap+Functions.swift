import SargonUniFFI

extension ComponentAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newComponentAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }

	public var networkID: NetworkId {
        self.networkId
	}

	/// If the `EntityType == .globalGenericComponent`
	public var isGlobal: Bool {
        self.kind == .global
	}
	
	/// If the `EntityType == .InternalGenericComponent`
	public var isInternal: Bool {
        self.kind == .internal
	}
}

#if DEBUG
extension ComponentAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newComponentAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		componentAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
