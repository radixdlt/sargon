extension NonFungibleGlobalID {
    public init(string: String) throws {
        self = try newNonFungibleGlobalIdFromString(string: string)
    }
}
