extension IdentityPath: SargonModel, CAP26PathProtocol {}

extension IdentityPath {
    public init(string: String) throws {
        switch try CAP26Path(string: string) {
        case let .identity(value):
            self = value
        case .account, .getId:
            throw SargonError.WrongEntityKind(message: "Expected Identity")
        }
    }
    
    public func embed() -> CAP26Path {
        .identity(value: self)
    }
}
