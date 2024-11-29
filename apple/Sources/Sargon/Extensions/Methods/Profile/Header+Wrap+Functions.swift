import Foundation
import SargonUniFFI

extension Header {
	public init(jsonData: some DataProtocol) throws {
		self = try newHeaderFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		headerToJsonBytes(header: self)
	}
}
