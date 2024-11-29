import SargonUniFFI

extension Unhardened {
	public static let globalOffset: UInt32 = 0

	public init(u31: U31) {
		self = newUnhardened(u31: u31)
	}

	public init(localKeySpace: UInt32) throws {
		self = try newUnhardenedFromLocalKeySpace(value: localKeySpace)
	}

	public init(globalKeySpace: UInt32) throws {
		self = try newUnhardenedFromGlobalKeySpace(value: globalKeySpace)
	}

	public func indexInLocalKeySpace() -> UInt32 {
		unhardenedIndexInLocalKeySpace(unhardened: self)
	}

	public func indexInGlobalKeySpace() -> UInt32 {
		unhardenedIndexInGlobalKeySpace(unhardened: self)
	}
}
