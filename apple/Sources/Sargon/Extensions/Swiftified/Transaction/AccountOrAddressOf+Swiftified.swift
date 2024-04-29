import SargonUniFFI

// MARK: - AccountOrAddressOf + SargonModel
extension AccountOrAddressOf: SargonModel {}

// MARK: - AccountOrAddressOf + CustomStringConvertible
extension AccountOrAddressOf: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

// MARK: - AccountOrAddressOf + Identifiable
extension AccountOrAddressOf: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		accountAddress
	}
}
