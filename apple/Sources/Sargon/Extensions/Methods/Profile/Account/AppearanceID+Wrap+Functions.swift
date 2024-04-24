import SargonUniFFI

extension AppearanceID: CaseIterable {
	public static var allCases: [Self] {
		appearanceIdsAll()
	}
	
	public static func fromNumberOfAccounts(_ numberOfAccounts: Int) -> Self {
		newAppearanceIdFromNumberOfAccountsOnNetwork(count: UInt64(numberOfAccounts))
	}
	
}
