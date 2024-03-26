public typealias CAP26Path = Cap26Path

extension CAP26Path: @unchecked Sendable {}
extension CAP26Path: SargonModel, CAP26PathProtocol {
    public func embed() -> CAP26Path {
        self
    }
}

