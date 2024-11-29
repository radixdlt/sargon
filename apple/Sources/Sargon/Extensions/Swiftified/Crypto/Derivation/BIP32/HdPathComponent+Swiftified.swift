import SargonUniFFI

// MARK: - HdPathComponent + BaseHDPathComponentProtocol
extension HdPathComponent: BaseHDPathComponentProtocol {}

// MARK: - HdPathComponent + CustomDebugStringConvertible
extension HdPathComponent: CustomDebugStringConvertible {
	public var debugDescription: String {
		toBIP32String()
	}
}

// MARK: - HdPathComponent + CustomStringConvertible
extension HdPathComponent: CustomStringConvertible {
	public var description: String {
		toBIP32StringDebug()
	}
}

extension HdPathComponent {
	public func asHardened() throws -> Hardened {
		try hdPathComponentToHardened(component: self)
	}
}
