import SargonUniFFI

extension PoolAddress: AddressProtocol {
	public var asGeneral: Address {
		.pool(self)
	}
}
