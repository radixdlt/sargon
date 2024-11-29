import Foundation
import SargonUniFFI

extension URLRequest {
	init(sargon: NetworkRequest) {
		var request = URLRequest(url: sargon.url)
		request.httpMethod = sargon.method.toString()
		request.httpBody = sargon.body
		request.allHTTPHeaderFields = sargon.headers
		self = request
	}
}
