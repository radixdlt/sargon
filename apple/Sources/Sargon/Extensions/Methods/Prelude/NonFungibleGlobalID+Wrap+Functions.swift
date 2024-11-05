import SargonUniFFI

extension NonFungibleGlobalID {
    public init(_ string: String) throws {
        self = try newNonFungibleGlobalIdFromString(string: string)
    }
    
    public func toRawString() -> String {
        self.asString
    }
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted.getString(format)
    }
}

extension FormattedAddress {
    func getString(_ format: AddressFormat) -> String {
        switch format {
        case .default:
            self.default
        case .full:
            self.full
        case .raw:
            self.raw
        }
    }
}
