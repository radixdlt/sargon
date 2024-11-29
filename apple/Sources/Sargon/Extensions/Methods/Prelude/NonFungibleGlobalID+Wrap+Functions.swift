import SargonUniFFI

extension NonFungibleGlobalID {
	public init(_ string: String) throws {
		self = try newNonFungibleGlobalIdFromString(string: string)
	}

	public func toRawString() -> String {
		nonFungibleGlobalIdToString(globalId: self)
	}

	public func formatted(_ format: AddressFormat = .default) -> String {
		nonFungibleGlobalIdFormatted(globalId: self, format: format)
	}
}
