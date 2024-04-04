import SargonUniFFI

extension PoolAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newPoolAddress(bech32: bech32String)
	}

    public func formatted(_ format: AddressFormat = .default) -> String {
        poolAddressFormatted(address: self, format: format)
    }
    
	/// The bech32 encoded string for this address.
	public var address: String {
		poolAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		poolAddressNetworkId(address: self)
	}

	/// Returns the kind of pool, either 1, 2 or Multi resources.
	public var poolKind: PoolKind {
		poolAddressKind(address: self)
	}

}

#if DEBUG
extension PoolAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newPoolAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		poolAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
