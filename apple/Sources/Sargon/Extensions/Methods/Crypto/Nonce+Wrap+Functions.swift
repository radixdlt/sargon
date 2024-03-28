import SargonUniFFI

extension Nonce {
    public static func secureRandom() -> Self {
        newNonceRandom()
    }
    
    public var value: UInt32 {
        nonceGetValue(nonce: self)
    }
}
