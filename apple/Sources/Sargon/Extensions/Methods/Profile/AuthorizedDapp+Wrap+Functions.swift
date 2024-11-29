import Foundation
import SargonUniFFI

extension AuthorizedDapp {
	public init(jsonData: some DataProtocol) throws {
		self = try newAuthorizedDappFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		authorizedDappToJsonBytes(authorizedDapp: self)
	}

	public mutating func showDeposits(_ show: Bool) {
		preferences.deposits = show ? .visible : .hidden
	}

	public var isDepositsVisible: Bool {
		preferences.deposits == .visible
	}
}
