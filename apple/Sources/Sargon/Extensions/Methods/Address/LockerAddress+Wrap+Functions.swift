import SargonUniFFI

extension LockerAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newLockerAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }

	public var networkID: NetworkId {
        self.networkId
	}

}

#if DEBUG
extension LockerAddress {
	
	public static func random(networkID: NetworkID) -> Self {
		newLockerAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		lockerAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
