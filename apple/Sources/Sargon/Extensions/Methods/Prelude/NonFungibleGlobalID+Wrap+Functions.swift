extension NonFungibleGlobalID {
    public init(string: String) throws {
        self = try newNonFungibleGlobalIdFromString(string: string)
    }
    
    public func toString() -> String {
        nonFungibleGlobalIdToString(globalId: self)
    }
}
