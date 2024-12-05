import Foundation
import SargonUniFFI

public typealias SargonError = CommonError

// MARK: - SargonError + SargonModel
extension SargonError: SargonModel {}

// MARK: - SargonError + CustomDebugStringConvertible
extension SargonError: CustomDebugStringConvertible {
	public var debugDescription: String {
        errorDescription()
	}
}

// MARK: - SargonError + CustomStringConvertible
extension SargonError: CustomStringConvertible {
	public var description: String {
        errorDescription()
	}
}

extension SargonError {
	public func errorDescription() -> String {
		let errorCodeFormatted = "Error code: \(errorCode)"

		var errorMessageFormatted: String?
		#if DEBUG
		errorMessageFormatted = "Error message: \(errorMessage)"
		#endif

		return [errorCodeFormatted, errorMessageFormatted]
			.compactMap { $0 }
			.joined(separator: "\n")
	}
}
