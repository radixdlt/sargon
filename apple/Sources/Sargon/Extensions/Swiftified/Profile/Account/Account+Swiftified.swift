import SargonUniFFI

extension Account: EntityProtocol {}

extension Account {
	public var appearanceID: AppearanceID {
		appearanceId
	}
}
