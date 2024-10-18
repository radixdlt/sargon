import SargonUniFFI

extension SignedTransactionIntentHash {
    public init(_ string: String) throws {
        self = try newSignedTransactionIntentHashFromString(string: string)
    }
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        signedTransactionIntentHashFormatted(address: self, format: format)
    }
}
