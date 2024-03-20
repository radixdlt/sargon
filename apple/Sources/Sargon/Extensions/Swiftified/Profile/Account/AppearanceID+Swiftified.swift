public typealias AppearanceID = AppearanceId

extension AppearanceID: @unchecked Sendable {}
extension AppearanceID: Identifiable {
	public typealias ID = UInt8
	public var id: ID {
		value
	}
}
extension AppearanceID: CustomStringConvertible {
	public var description: String {
		value.description
	}
}
