extension IntentHash: SargonModel {}

extension IntentHash: CustomStringConvertible {
    public var description: String {
        self.bech32EncodedTxId
    }
    
    public var networkID: NetworkID {
        networkId
    }
}
