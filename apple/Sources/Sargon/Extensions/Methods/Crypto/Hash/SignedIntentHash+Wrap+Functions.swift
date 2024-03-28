import SargonUniFFI

extension SignedIntentHash {
    public init(string: String) throws {
        self = try newSignedIntentHashFromString(string: string)
    }
}
