import Foundation
import SargonUniFFI

extension AuthorizedDapp {
	public init(jsonData: some DataProtocol) throws {
		self = try newAuthorizedDappFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		authorizedDappToJsonBytes(authorizedDapp: self)
	}
}
