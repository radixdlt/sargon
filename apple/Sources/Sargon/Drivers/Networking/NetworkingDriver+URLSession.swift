import Foundation
import SargonUniFFI

// Makes it possible to type `.shared` on an initalizer/func taking
// `some NetworkingDriver` as parameter.
extension NetworkingDriver where Self == URLSession {
	/// Singleton `NetworkingDriver` of type `URLSession`, which
	/// uses `URLSession.shared`.
	public static var shared: Self { Self.shared }
}

// MARK: - URLSession + NetworkingDriver
extension URLSession: NetworkingDriver {
	public func executeNetworkRequest(
		request sargonRequest: NetworkRequest
	) async throws -> NetworkResponse {
		let request = URLRequest(sargon: sargonRequest)
		let response: (Data, URLResponse)
		do {
			response = try await data(for: request)
		} catch {
			throw SargonError.NetworkRequestGenericFailure(
				underlying: String(describing: error))
		}
		return try NetworkResponse(response: response)
	}
}
