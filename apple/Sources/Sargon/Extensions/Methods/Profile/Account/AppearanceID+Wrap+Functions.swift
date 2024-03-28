import SargonUniFFI

extension AppearanceID: CaseIterable {
	public static var allCases: [Self] {
		appearanceIdsAll()
	}
}
