import SargonUniFFI

extension NonFungibleGlobalID {
    public init(string: String) throws {
        self = try newNonFungibleGlobalIdFromString(string: string)
    }
    
    public func toString() -> String {
        nonFungibleGlobalIdToString(globalId: self)
    }
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        nonFungibleGlobalIdFormatted(globalId: self, format: format)
    }
}
