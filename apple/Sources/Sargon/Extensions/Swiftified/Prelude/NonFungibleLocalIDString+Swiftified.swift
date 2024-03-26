public typealias NonFungibleLocalIDString = NonFungibleLocalIdString
extension NonFungibleLocalIDString: @unchecked Sendable {}
extension NonFungibleLocalIDString: SargonModel {}

#if DEBUG
extension NonFungibleLocalIDString: ExpressibleByStringLiteral {
    public init(stringLiteral value: String) {
        try! self.init(validating: value)
    }
}
#endif
