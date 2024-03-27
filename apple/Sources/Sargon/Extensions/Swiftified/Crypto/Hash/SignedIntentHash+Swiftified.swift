extension SignedIntentHash: SargonModel {}

extension SignedIntentHash: CustomStringConvertible {
    public var description: String {
        self.bech32EncodedTxId
    }
    
    public var networkID: NetworkID {
        networkId
    }
}
