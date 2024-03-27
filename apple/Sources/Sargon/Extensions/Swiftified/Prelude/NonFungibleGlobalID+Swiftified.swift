public typealias NonFungibleGlobalID = NonFungibleGlobalId
extension NonFungibleGlobalID: SargonModel {}

extension NonFungibleGlobalID {
    public init(
        nonFungibleResourceAddress: NonFungibleResourceAddress,
        localID: NonFungibleLocalID
    ) {
        self.init(
            resourceAddress: nonFungibleResourceAddress.asResourceAddress,
            nonFungibleLocalId: localID
        )
    }
}

extension NonFungibleGlobalID: CustomStringConvertible {
    public var description: String {
        toString()
    }
}

extension NonFungibleGlobalID: Identifiable {
    public typealias ID = String
    public var id: String {
        toString()
    }
}

#if DEBUG
extension NonFungibleGlobalID: ExpressibleByStringLiteral {
    public init(stringLiteral value: String) {
        try! self.init(string: value)
    }
}
#endif // DEBUG
