
extension NetworkID {
	
	public init(discriminant: UInt8) throws {
		self = try newNetworkIdFromDiscriminant(discriminant: discriminant)
	}
	
	public var description: String {
		networkIdToString(id: self)
	}
}
