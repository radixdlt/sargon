import SargonUniFFI

extension NetworkID: CaseIterable {
	public init(discriminant: UInt8) throws {
		self = try newNetworkIdFromDiscriminant(discriminant: discriminant)
	}
	
	public static var allCases: [Self] {
		networkIdsAll()
	}
	
	public func toString() -> String {
		networkIdToString(id: self)
		
	}
}
