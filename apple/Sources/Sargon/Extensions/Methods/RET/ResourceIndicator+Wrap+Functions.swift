import SargonUniFFI

extension ResourceIndicator {
	public var resourceAddress: ResourceAddress {
		resourceIndicatorGetAddress(indicator: self)
	}
}
