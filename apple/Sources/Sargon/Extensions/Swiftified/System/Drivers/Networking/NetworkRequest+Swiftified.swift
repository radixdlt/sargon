import Foundation
import SargonUniFFI

extension URL {
	public init(validating string: String) throws {
		guard let url = Self(string: string) else {
			throw SargonError.InvalidUrl(badValue: string)
		}
		self = url
	}
}

extension NetworkRequest {
	public init(
		validating urlString: String,
		method: NetworkMethod,
		headers: [String: String] = [:],
		body: Data = .init()
	) throws {
		let url = try URL(
			validating: urlString
		)
		self.init(
			url: url,
			method: method,
			headers: headers,
			body: body
		)
	}
}
