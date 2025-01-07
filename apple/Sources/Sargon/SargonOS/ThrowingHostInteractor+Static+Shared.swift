// A default stub implementation of the HostInteractor, that always rejects any operation
public class ThrowingHostInteractor: HostInteractor {
	public nonisolated(unsafe) static var shared: HostInteractor = ThrowingHostInteractor()

	public func signAuth(request: SargonUniFFI.SignRequestOfAuthIntent) async throws -> SargonUniFFI.SignResponseOfAuthIntentHash {
		throw CommonError.SigningRejected
	}

	public func signTransactions(request: SargonUniFFI.SignRequestOfTransactionIntent) async throws -> SargonUniFFI.SignResponseOfTransactionIntentHash {
		throw CommonError.SigningRejected
	}

	public func signSubintents(request: SargonUniFFI.SignRequestOfSubintent) async throws -> SargonUniFFI.SignResponseOfSubintentHash {
		throw CommonError.SigningRejected
	}

	public func deriveKeys(request: SargonUniFFI.KeyDerivationRequest) async throws -> SargonUniFFI.KeyDerivationResponse {
		throw CommonError.SigningRejected
	}
}
