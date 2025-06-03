import SargonUniFFI

// MARK: - AccountOrAddressOf + SargonModel
extension TransferRecipient: SargonModel {}

// MARK: - AccountOrAddressOf + CustomStringConvertible
extension TransferRecipient: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

// MARK: - AccountOrAddressOf + Identifiable
extension TransferRecipient: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		accountAddress
	}
}
