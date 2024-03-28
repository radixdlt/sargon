import SargonUniFFI

extension Exactly33Bytes {
	
	public init(bytes: some DataProtocol) throws {
		self = try newExactly33Bytes(bytes: BagOfBytes(bytes))
	}
	
	public var data: Data {
		exactly33BytesToBytes(bytes: self)
	}
	
	public var hex: String {
		exactly33BytesToHex(bytes: self)
	}
}

