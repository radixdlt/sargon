import Foundation
import SargonUniFFI

public typealias NetworkID = NetworkId

extension NetworkID: SargonModel {}

extension NetworkID: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

extension NetworkID: Codable {
	public func encode(to encoder: Encoder) throws {
		var container = encoder.singleValueContainer()
		try container.encode(self.rawValue)
	}

	public init(from decoder: Decoder) throws {
		let container = try decoder.singleValueContainer()
		let discriminant = try container.decode(UInt8.self)
		try self.init(discriminant: discriminant)
	}
}
