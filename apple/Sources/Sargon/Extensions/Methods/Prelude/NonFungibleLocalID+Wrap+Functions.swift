extension NonFungibleLocalID {
	public func toString() -> String {
		nonFungibleLocalIdAsStr(id: self)
	}
	
	public init(integer value: UInt64) {
		self = newNonFungibleLocalIdInt(value: value)
	}
	
	/// Tries to decode an String as NonFungibleLocalID.string
	public init(string: String) throws {
		self = try newNonFungibleLocalIdString(string: string)
	}
	
	public init(bytes: some DataProtocol) throws {
		self = try newNonFungibleLocalIdBytes(bytes: Data(bytes))
	}
	
	public init(ruid ruidBytes: some DataProtocol) throws {
		self = try newNonFungibleLocalIdRuid(bytes: Data(ruidBytes))
	}
}
