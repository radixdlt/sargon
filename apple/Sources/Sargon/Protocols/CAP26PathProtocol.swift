public protocol CAP26PathProtocol: HDPathProtocol {
    var asGeneral: CAP26Path { get }
}

extension CAP26PathProtocol {
    public var description: String {
        asGeneral.toString()
    }
}
