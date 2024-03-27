extension ResourceAddress: AddressProtocol {}

extension ResourceAddress {

    public var isXRD: Bool {
        self == self.xrdOnSameNetwork
    }
}
