import SargonUniFFI

extension DataProtocol {
	public func hash() -> Exactly32Bytes {
		SargonUniFFI.hash(data: Data(self))
	}
}

extension Hash {
    public var data: Data {
        hashGetBytes(hash: self)
    }
    
}
