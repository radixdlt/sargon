
#if DEBUG
public protocol BaseHDPathProtocol: BaseSargonModel, CustomStringConvertible, ExpressibleByStringLiteral {}
#else
public protocol BaseHDPathProtocol: BaseSargonModel, CustomStringConvertible {}
#endif // DEBUG

public protocol HDPathProtocol: BaseHDPathProtocol {
    init(string: String) throws
    func toString() -> String
}

#if DEBUG
extension HDPathProtocol {
    public init(stringLiteral value: String) {
        try! self.init(string: value)
    }
}
#endif

