import Foundation
import JSONValue
import AnyCodable

public protocol SargonCodable: Codable {
	init(jsonString: String) throws
	func jsonString() -> String
}

extension SargonCodable {
	public func encode(to encoder: any Encoder) throws {
		let jsonString = self.jsonString()
		let jsonData = jsonString.data(using: .utf8)!
		let value = try JSONDecoder().decode(JSONValue.self, from: jsonData)
		var container = encoder.singleValueContainer()
		try container.encode(value)
	}
	
	public init(from decoder: any Decoder) throws {
		let container = try decoder.singleValueContainer()
		let jsonValue = try container.decode(JSONValue.self)
		let jsonData = try JSONEncoder().encode(jsonValue)
		let jsonString = String(data: jsonData, encoding: .utf8)!
		try self.init(jsonString: jsonString)
	}
}


extension DeviceInfo: SargonModel {}

extension DeviceInfo: SargonCodable {}
