import SargonUniFFI

extension Blob {

	public init(data: some DataProtocol) {
		self = newBlobFromBytes(bytes: Data(data))
	}
	
	public var data: Data {
		blobToBytes(blob: self)
	}
	
	public var hex: String {
		blobToString(blob: self)
	}
}
