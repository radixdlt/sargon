import Foundation
import SwiftyJSON

public protocol SargonObjectCodable: Codable {
	init(jsonData: some DataProtocol) throws
	func jsonData() -> Data
}

extension SargonObjectCodable {
	public func encode(to encoder: any Encoder) throws {
		var container = encoder.singleValueContainer()
		let json = try JSON(data: jsonData())
		try container.encode(json)
	}
	
	public init(from decoder: any Decoder) throws {
		let container = try decoder.singleValueContainer()
		let json = try container.decode(JSON.self)
		try self.init(jsonData: json.rawData())
	}
}

