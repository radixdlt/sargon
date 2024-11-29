import SargonUniFFI

extension HdPathComponent {
	public func toBIP32String() -> String {
		hdPathComponentToBip32String(component: self)
	}

	public func toBIP32StringDebug() -> String {
		hdPathComponentToBip32StringDebug(component: self)
	}

	public init(globalKeySpace: UInt32) {
		self = newHdPathComponentFromGlobalKeySpace(value: globalKeySpace)
	}

	public init(localKeySpace: UInt32, keySpace: KeySpace) throws {
		self = try newHdPathComponentFromLocalKeySpace(value: localKeySpace, keySpace: keySpace)
	}

	public var keySpace: KeySpace {
		hdPathComponentGetKeySpace(component: self)
	}

	public func indexInGlobalKeySpace() -> UInt32 {
		hdPathComponentIndexInGlobalKeySpace(component: self)
	}

	public func indexInLocalKeySpace() -> UInt32 {
		hdPathComponentIndexInLocalKeySpace(component: self)
	}
}
