import SargonUniFFI

// MARK: - TransferRecipient + SargonModel
extension TransferRecipient: SargonModel {}

// MARK: - TransferRecipient + CustomStringConvertible
extension TransferRecipient: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

// MARK: - TransferRecipient + Identifiable
extension TransferRecipient: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		accountAddress
	}
}
