import SargonUniFFI

extension LockerAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newLockerAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        lockerAddressFormatted(address: self, format: format)
    }

	/// The bech32 encoded string for this address.
	public var address: String {
		lockerAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		lockerAddressNetworkId(address: self)
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
