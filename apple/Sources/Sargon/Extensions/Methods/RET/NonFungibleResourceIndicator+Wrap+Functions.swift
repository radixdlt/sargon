import SargonUniFFI

extension NonFungibleResourceIndicator {
	public var ids: [NonFungibleLocalId] {
		nonFungibleResourceIndicatorGetIds(indicator: self)
	}
}
