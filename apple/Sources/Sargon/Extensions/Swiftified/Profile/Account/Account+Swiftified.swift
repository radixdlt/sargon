import SargonUniFFI

// MARK: - Account + EntityBaseProtocol
extension Account: EntityBaseProtocol {
	public typealias EntityAddress = AccountAddress

	public var asGeneral: AccountOrPersona {
		.account(self)
	}

	public var unsecuredControllingFactorInstance: SargonUniFFI.HierarchicalDeterministicFactorInstance? {
		accountUnsecuredControllingFactorInstance(account: self)
	}
}

extension Account {
	public var appearanceID: AppearanceID {
		appearanceId
	}

	public var forDisplay: AccountForDisplay {
		AccountForDisplay(address: address, displayName: displayName, appearanceId: appearanceID)
	}
}

// MARK: - Account + EntityProtocol
extension Account: EntityProtocol {
	public static let kind: EntityKind = .account

	public static func extract(from someEntity: some EntityBaseProtocol) -> Self? {
		guard case let .accountEntity(account) = someEntity.asGeneral else { return nil }
		return account
	}

	public struct ExtraProperties: SargonModel {
		public var appearanceID: AppearanceID
		public let onLedgerSettings: OnLedgerSettings

		public init(
			appearanceID: AppearanceID,
			onLedgerSettings: OnLedgerSettings = .default
		) {
			self.appearanceID = appearanceID
			self.onLedgerSettings = onLedgerSettings
		}
	}

	public init(
		networkID: NetworkID,
		address: AccountAddress,
		securityState: EntitySecurityState,
		displayName: DisplayName,
		extraProperties: ExtraProperties
	) {
		self.init(
			networkId: networkID,
			address: address,
			displayName: displayName,
			securityState: securityState,
			appearanceId: extraProperties.appearanceID,
			flags: [],
			onLedgerSettings: extraProperties.onLedgerSettings
		)
	}

	public static func deriveVirtualAddress(
		networkID: NetworkID,
		factorInstance: HierarchicalDeterministicFactorInstance
	) -> AccountAddress {
		AccountAddress(publicKey: factorInstance.publicKey.publicKey, networkID: networkID)
	}
}

#if DEBUG
extension Account.ExtraProperties {
	public static let sample = Self(appearanceID: .sample, onLedgerSettings: .sample)
	public static let sampleOther = Self(appearanceID: .sampleOther, onLedgerSettings: .sampleOther)
}
#endif // DEBUG
