import SargonUniFFI

extension AccountOrAddressOf: SargonModel {}

extension AccountOrAddressOf: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

extension AccountOrAddressOf: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		accountAddress
	}
}
