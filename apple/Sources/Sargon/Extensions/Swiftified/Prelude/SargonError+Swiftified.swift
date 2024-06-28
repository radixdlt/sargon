import Foundation
import SargonUniFFI

public typealias SargonError = CommonError

extension SargonError: SargonModel {}

extension SargonError: CustomDebugStringConvertible {
	public var debugDescription: String {
		"\(errorCode): \(errorMessage)"
	}
}

extension SargonError: CustomStringConvertible {
	public var description: String {
		errorMessage
	}
}

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
