import SargonUniFFI

extension SargonError {
	public var errorMessage: String {
		errorMessageFromError(error: self)
	}

	public var errorCode: UInt32 {
		errorCodeFromError(error: self)
	}
}
