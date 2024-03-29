import SargonUniFFI

extension NonFungibleLocalID {
	public func toRawString() -> String {
		nonFungibleLocalIdAsStr(id: self)
	}
	
	public init(integer value: UInt64) {
		self = newNonFungibleLocalIdInt(value: value)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        nonFungibleLocalIdFormatted(id: self, format: format)
    }
    
    public func toUserFacingString() -> String {
        nonFungibleLocalIdToUserFacingString(id: self)
    }
	
    /// Tries to decode an String representation of any NonFungibleLocalID, either:
    /// * integer
    /// * bytes
    /// * ruid
    /// * string
    ///
    /// Not to be confused with `NonFungibleLocalID.stringID` which tries to decode
	/// a `NonFungibleLocalID.string` variant
    public init(_ string: String) throws {
        self = try newNonFungibleLocalIdFromString(localId: string)
    }
    
    /// Tries to decode a `NonFungibleLocalID.string`
    ///
    /// Not to be confused with `init(:String)` which tries to decode
    /// a string into any case of `NonFungibleLocalID`
	public static func stringID(_ string: String) throws -> Self {
		try newNonFungibleLocalIdString(string: string)
	}
	
	public init(bytes: some DataProtocol) throws {
		self = try newNonFungibleLocalIdBytes(bytes: Data(bytes))
	}
	
	public init(ruid ruidBytes: some DataProtocol) throws {
		self = try newNonFungibleLocalIdRuid(bytes: Data(ruidBytes))
	}
}
