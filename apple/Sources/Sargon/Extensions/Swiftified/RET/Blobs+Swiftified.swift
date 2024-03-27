extension Blobs: SargonModel {}

extension Blobs: ExpressibleByArrayLiteral {
	public typealias ArrayLiteralElement = Blob
	public init(arrayLiteral blobs: ArrayLiteralElement...) {
		self.init(blobs)
	}
}
