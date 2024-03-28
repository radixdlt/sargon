extension ResourceAddress: AddressProtocol {}

extension ResourceAddress {

    public var isXRD: Bool {
        self == self.xrdOnSameNetwork
    }
    
    public var asNonFungibleResourceAddress: NonFungibleResourceAddress? {
        try? NonFungibleResourceAddress(validatingAddress: address)
    }
}
