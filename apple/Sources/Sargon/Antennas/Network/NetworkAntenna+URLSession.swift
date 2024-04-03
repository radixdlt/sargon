import Foundation
import SargonUniFFI

extension URLRequest {
	init(sargon: NetworkRequest) {
		var request = URLRequest(url: sargon.url)
		switch sargon.method {
		case .post:
			request.httpMethod = "POST"
		}
		
		request.httpBody = sargon.body
		request.allHTTPHeaderFields = sargon.headers
		self = request
	}
}

extension NetworkResponse {
	init(response: (Data, URLResponse)) throws {
		guard let httpURLResponse = response.1 as? HTTPURLResponse else {
			throw SargonError.Unknown(message: "Expected HTTPURLResponse")
		}
		self.init(
			statusCode: UInt16(httpURLResponse.statusCode),
			body: response.0
		)
	}
}

extension URLSession: NetworkAntenna {
	
	public func makeRequest(
		request sargonRequest: NetworkRequest
	) async throws -> NetworkResponse {
		let request = URLRequest(sargon: sargonRequest)
		let response = try await data(for: request)
		return try NetworkResponse(response: response)
	}
}
