extension ResourceAddress: @unchecked Sendable {}

extension ResourceAddress {

    public var isXRD: Bool {
        self == self.xrdOnSameNetwork
    }
}
