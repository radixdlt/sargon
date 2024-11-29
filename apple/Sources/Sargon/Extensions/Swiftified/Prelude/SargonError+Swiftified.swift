import Foundation
import SargonUniFFI

public typealias SargonError = CommonError

// MARK: - SargonError + SargonModel
extension SargonError: SargonModel {}

// MARK: - SargonError + CustomDebugStringConvertible
extension SargonError: CustomDebugStringConvertible {
	public var debugDescription: String {
		"\(errorCode): \(errorMessage)"
	}
}

// MARK: - SargonError + CustomStringConvertible
extension SargonError: CustomStringConvertible {
	public var description: String {
		errorMessage
	}
}

// MARK: - SargonError + LocalizedError
extension SargonError: LocalizedError {
	public var errorDescription: String? {
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
