public typealias AppearanceID = AppearanceId

extension AppearanceID: Sendable {}
extension AppearanceID: CaseIterable {
	public static var allCases: [Self] {
		appearanceIdsAll()
	}
}
