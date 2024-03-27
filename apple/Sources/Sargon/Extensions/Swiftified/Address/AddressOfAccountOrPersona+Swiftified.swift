extension AddressOfAccountOrPersona: AddressProtocol {
    public func formatted(_ format: AddressFormat) -> String {
        switch self {
        case let .account(address): address.formatted(format)
        case let .persona(address): address.formatted(format)
        }
    }
}
