extension ResourceAddress: AddressProtocol {}

extension ResourceAddress {

    public var isXRD: Bool {
        self == self.xrdOnSameNetwork
    }
    
    public var asNonFungibleResourceAddress: NonFungibleResourceAddress? {
		try? NonFungibleResourceAddress(validatingAddress: address)
	}
	
	public func isXRD(on networkID: NetworkID) -> Bool {
		self == Self.xrd(on: networkID)
	}
	
	/// The ResourceAddress of XRD of mainnet
	public static let mainnetXRD = Self.xrd(on: .mainnet)
}
