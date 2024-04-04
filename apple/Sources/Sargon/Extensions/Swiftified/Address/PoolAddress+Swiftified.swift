import SargonUniFFI

extension PoolAddress: AddressProtocol {
	public func embed() -> Address {
		.pool(self)
	}
}
