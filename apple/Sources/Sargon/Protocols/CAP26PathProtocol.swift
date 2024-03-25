public protocol CAP26PathProtocol: HDPathProtocol {
    func embed() -> CAP26Path
}

extension CAP26PathProtocol {
    public var description: String {
        embed().toString()
    }
}
