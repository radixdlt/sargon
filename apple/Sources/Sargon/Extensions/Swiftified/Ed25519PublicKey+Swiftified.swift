extension Ed25519PublicKey: @unchecked Sendable {}

extension Ed25519PublicKey: SargonModel {
    public var description: String {
        hex
    }
}
