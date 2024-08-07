import SargonUniFFI

public typealias NonFungibleGlobalID = NonFungibleGlobalId

extension NonFungibleGlobalID: IdentifiableByStringProtocol {
    public var localID: NonFungibleLocalID {
        nonFungibleLocalId
    }
}

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
