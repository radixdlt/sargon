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
		"\(errorMessage)\nCode: \(errorCode)"
	}
}