import Foundation
import SargonUniFFI

extension URLRequest {
	init(sargon: NetworkRequest) {
		var request = URLRequest(url: sargon.url)
		switch sargon.method {
		case .post:
			request.httpMethod = "POST" // FIXME: embed in sargon
		case .get:
			request.httpMethod = "GET"
		}

		request.httpBody = sargon.body
		request.allHTTPHeaderFields = sargon.headers
		self = request
	}
}

extension NetworkResponse {
	init(response: (Data, URLResponse)) throws {
		guard let httpURLResponse = response.1 as? HTTPURLResponse else {
			throw SargonError.NetworkRequestGenericFailure(
				underlying: "Failed to cast to HTTPURLResponse")
		}
		self.init(
			statusCode: UInt16(httpURLResponse.statusCode),
			body: response.0
		)
	}
}

// MARK: - URLSession + NetworkAntenna
extension URLSession: NetworkAntenna {
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
