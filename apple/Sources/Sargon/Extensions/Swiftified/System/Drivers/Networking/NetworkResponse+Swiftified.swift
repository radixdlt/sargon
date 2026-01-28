import Foundation
import SargonUniFFI

extension NetworkResponse {
	init(response: (Data, URLResponse)) throws {
		guard let httpURLResponse = response.1 as? HTTPURLResponse else {
			throw SargonError.NetworkRequestGenericFailure(
				underlying: "Failed to cast to HTTPURLResponse"
			)
		}
		self.init(
			statusCode: UInt16(httpURLResponse.statusCode),
			body: response.0
		)
	}
}
