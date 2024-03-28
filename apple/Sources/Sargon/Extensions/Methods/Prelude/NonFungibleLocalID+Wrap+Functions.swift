import SargonUniFFI

extension NonFungibleLocalID {
	public func toString() -> String {
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
    /// Not to be confused with `init(string:)` which tries to decode
    /// `NonFungibleLocalID.string`
    public init(localId: String) throws {
        self = try newNonFungibleLocalIdFromString(localId: localId)
    }
    
    /// Tries to decode a `NonFungibleLocalID.string`
    ///
    /// Not to be confused with `init(localId:)` which tries to decode
    /// a string into any case of `NonFungibleLocalID` 
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
