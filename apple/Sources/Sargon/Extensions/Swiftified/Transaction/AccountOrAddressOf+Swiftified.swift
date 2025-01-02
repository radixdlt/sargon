import SargonUniFFI

// MARK: - OwnedOrThirdPartyAccountAddress + SargonModel
extension OwnedOrThirdPartyAccountAddress: SargonModel {}

// MARK: - OwnedOrThirdPartyAccountAddress + CustomStringConvertible
extension OwnedOrThirdPartyAccountAddress: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

// MARK: - OwnedOrThirdPartyAccountAddress + Identifiable
extension OwnedOrThirdPartyAccountAddress: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		accountAddress
	}
}
