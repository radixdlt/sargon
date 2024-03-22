
extension NetworkID {
	public init(discriminant: UInt8) throws {
		self = try newNetworkIdFromDiscriminant(discriminant: discriminant)
	}
	
	public static var allCases: [Self] {
		networkIdsAll()
	}
}

extension NetworkID: CustomStringConvertible {
	public var description: String {
		networkIdToString(id: self)
	}
}

