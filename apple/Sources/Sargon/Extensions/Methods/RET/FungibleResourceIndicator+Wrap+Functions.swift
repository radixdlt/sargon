import SargonUniFFI

extension FungibleResourceIndicator {
	public var amount: Decimal192 {
		fungibleResourceIndicatorGetAmount(indicator: self)
	}
}
