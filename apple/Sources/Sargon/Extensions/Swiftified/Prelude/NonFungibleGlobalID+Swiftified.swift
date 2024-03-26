public typealias NonFungibleGlobalID = NonFungibleGlobalId
extension NonFungibleGlobalID: @unchecked Sendable {}
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


#if DEBUG
extension NonFungibleGlobalID: ExpressibleByStringLiteral {
    public init(stringLiteral value: String) {
        try! self.init(string: value)
    }
}
#endif // DEBUG
