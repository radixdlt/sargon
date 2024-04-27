import SargonUniFFI

extension Account: EntityProtocol {
	public typealias EntityAddress = AccountAddress
	
	public var asGeneral: AccountOrPersona {
		.account(self)
	}
}

extension Account {
	public var appearanceID: AppearanceID {
		appearanceId
	}
}

extension Account: EntitySpecificProtocol {
	public static let kind: EntityKind = .account
	public static func extract(from someEntity: some EntityProtocol) -> Self? {
		guard case let .account(account) = someEntity.asGeneral else { return nil }
		return account
	}
}
