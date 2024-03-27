extension AssetsTransfersRecipient: SargonModel {}

extension AssetsTransfersRecipient: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

extension AssetsTransfersRecipient: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		accountAddress
	}
}
