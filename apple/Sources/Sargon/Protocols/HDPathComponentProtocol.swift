// MARK: - BaseHDPathComponentProtocol
public protocol BaseHDPathComponentProtocol: SargonModel {
	init(globalKeySpace: UInt32) throws
	func indexInLocalKeySpace() -> UInt32
	func indexInGlobalKeySpace() -> UInt32
}

// MARK: - HDPathComponentProtocol
public protocol HDPathComponentProtocol: BaseHDPathComponentProtocol {
	static var globalOffset: UInt32 { get }
	init(localKeySpace: UInt32) throws
}
