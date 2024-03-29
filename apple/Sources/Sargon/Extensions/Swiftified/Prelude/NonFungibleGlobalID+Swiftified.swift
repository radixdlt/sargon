public typealias NonFungibleGlobalID = NonFungibleGlobalId

extension NonFungibleGlobalID: IdentifiableByStringProtocol {}

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
