import SargonUniFFI

public typealias NonFungibleGlobalID = NonFungibleGlobalId

extension NonFungibleGlobalID: IdentifiableByStringProtocol {
    public var localID: NonFungibleLocalID {
        nonFungibleLocalId
    }
}

extension NonFungibleGlobalID {
    public static func createWith(
        nonFungibleResourceAddress: NonFungibleResourceAddress,
        localID: NonFungibleLocalID
    ) -> Self {
        newNonFungibleGlobalId(address: nonFungibleResourceAddress, localId: localID)
    }

    public static func createWith(
        resourceAddress: ResourceAddress,
        localID: NonFungibleLocalID
    ) -> Self {
        newNonFungibleGlobalId(address: NonFungibleResourceAddress(value: resourceAddress), localId: localID)
    }
}
