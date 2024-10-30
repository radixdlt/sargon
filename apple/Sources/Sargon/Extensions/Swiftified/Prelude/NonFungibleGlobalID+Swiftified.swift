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
        nonFungibleLocalId: NonFungibleLocalID
    ) {
        self = newNonFungibleGlobalId(address: nonFungibleResourceAddress, localId: nonFungibleLocalId)
    }

    public init(
        resourceAddress: ResourceAddress,
        nonFungibleLocalId: NonFungibleLocalID
    ) {
        self.init(nonFungibleResourceAddress: NonFungibleResourceAddress(value: resourceAddress), nonFungibleLocalId: nonFungibleLocalId)
    }
}
