extension AccountPath: SargonModel, CAP26PathProtocol {}

extension AccountPath {
    public init(string: String) throws {
        switch try CAP26Path(string: string) {
        case let .account(value):
            self = value
        case .identity, .getId:
            throw SargonError.WrongEntityKind(message: "Expected Account")
        }
    }
    
    public func embed() -> CAP26Path {
        .account(value: self)
    }
}

