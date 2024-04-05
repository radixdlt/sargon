import SargonUniFFI

extension AddressOfAccountOrPersona: AddressProtocol {
	public var asGeneral: Address {
		switch self {
		case let .account(accountAddress): Address.account(accountAddress)
		case let .identity(identityAddress): Address.identity(identityAddress)
		}
	}
}
