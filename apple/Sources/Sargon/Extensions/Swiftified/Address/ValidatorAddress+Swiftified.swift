import SargonUniFFI

extension ValidatorAddress: AddressProtocol {
	public var asGeneral: Address {
		.validator(self)
	}
}
