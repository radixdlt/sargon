import SargonUniFFI

extension Exactly29Bytes {
	
	public init(bytes: some DataProtocol) throws {
		self = try newExactly29Bytes(bytes: BagOfBytes(bytes))
	}
	
	public var data: Data {
		exactly29BytesToBytes(bytes: self)
	}
	
	public var hex: String {
		exactly29BytesToHex(bytes: self)
	}
}

