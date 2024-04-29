
#if DEBUG
public protocol BaseHDPathProtocol: BaseSargonModel, CustomStringConvertible, ExpressibleByStringLiteral {}
#else
public protocol BaseHDPathProtocol: BaseSargonModel, CustomStringConvertible {}
#endif // DEBUG

// MARK: - HDPathProtocol
public protocol HDPathProtocol: BaseHDPathProtocol {
	init(string: String) throws
	var path: HdPath { get }
	func toString() -> String
}

#if DEBUG
extension HDPathProtocol {
	public init(stringLiteral value: String) {
		try! self.init(string: value)
	}
}
#endif
