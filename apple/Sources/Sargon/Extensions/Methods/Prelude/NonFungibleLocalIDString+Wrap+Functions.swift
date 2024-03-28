import SargonUniFFI

extension NonFungibleLocalIDString {
    public init(validating string: String) throws {
        self = try newNonFungibleLocalIdStringFromStr(string: string)
    }
}
