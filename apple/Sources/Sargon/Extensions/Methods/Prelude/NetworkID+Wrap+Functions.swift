
extension NetworkID {
	
	public init(discriminant: UInt8) throws {
		self = try newNetworkIdFromDiscriminant(discriminant: discriminant)
	}
}

extension NetworkID: CustomStringConvertible {
	public var description: String {
		networkIdToString(id: self)
	}
}
