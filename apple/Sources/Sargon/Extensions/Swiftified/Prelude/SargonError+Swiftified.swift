import SargonUniFFI

public typealias SargonError = CommonError

// MARK: SargonModel
extension SargonError: SargonModel {}

// MARK: CustomDebugStringConvertible
extension SargonError: CustomDebugStringConvertible {
	public var debugDescription: String {
		"\(errorCode): \(errorMessage)"
	}
}

// MARK: CustomStringConvertible
extension SargonError: CustomStringConvertible {
	public var description: String {
		errorMessage
	}
}
