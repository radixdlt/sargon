import SargonUniFFI

extension ValidatorAddress: AddressProtocol {
	public func embed() -> Address {
		.validator(self)
	}
}
