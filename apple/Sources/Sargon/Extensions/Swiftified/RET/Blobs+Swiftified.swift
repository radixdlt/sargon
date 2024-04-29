import SargonUniFFI

// MARK: - Blobs + SargonModel
extension Blobs: SargonModel {}

// MARK: - Blobs + ExpressibleByArrayLiteral
extension Blobs: ExpressibleByArrayLiteral {
	public typealias ArrayLiteralElement = Blob
	public init(arrayLiteral blobs: ArrayLiteralElement...) {
		self.init(blobs)
	}
}
