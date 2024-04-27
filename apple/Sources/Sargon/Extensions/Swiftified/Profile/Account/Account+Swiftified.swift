import SargonUniFFI

extension Account: EntityProtocol {
	public typealias EntityAddress = AccountAddress
}

extension Account {
	public var appearanceID: AppearanceID {
		appearanceId
	}
}
