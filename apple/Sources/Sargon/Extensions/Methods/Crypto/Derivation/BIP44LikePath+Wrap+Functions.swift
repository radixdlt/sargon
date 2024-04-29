import SargonUniFFI

public typealias HDPathValue = UInt32

extension BIP44LikePath {
	public init(string: String) throws {
		self = try newBip44LikePathFromString(string: string)
	}

	public func toString() -> String {
		bip44LikePathToString(path: self)
	}

	public init(index: HDPathValue) {
		self = newBip44LikePathFromIndex(index: index)
	}

	public var addressIndex: HDPathValue {
		bip44LikePathGetAddressIndex(path: self)
	}
}
