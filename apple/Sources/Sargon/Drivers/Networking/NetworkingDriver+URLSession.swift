import Foundation
import SargonUniFFI

extension NetworkingDriver where Self == URLSession {
	public static var shared: Self { Self.shared }
}

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
