import SargonUniFFI

// MARK: - Blob + SargonModel
extension Blob: SargonModel {}

// MARK: - Blob + CustomStringConvertible
extension Blob: CustomStringConvertible {
	public var description: String {
		hex
	}
}
