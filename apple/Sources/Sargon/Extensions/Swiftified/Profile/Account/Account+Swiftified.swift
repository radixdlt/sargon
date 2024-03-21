extension Account: @unchecked Sendable {}
extension Account: EntityProtocol {}

extension Account {
	public var appearanceID: AppearanceID {
		appearanceId
	}
}
