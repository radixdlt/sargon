import Foundation
import AnyCodable

public protocol SargonObjectCodable: Codable {
	init(jsonData: some DataProtocol) throws
	func jsonData() -> Data
}

extension SargonObjectCodable {
	public func encode(to encoder: any Encoder) throws {
		let dict = try JSONSerialization.jsonObject(with: self.jsonData(), options: []) as! NSDictionary
		let anyEncodable = AnyEncodable(dict)
		var container = encoder.singleValueContainer()
		try container.encode(anyEncodable)
	}
	
	public init(from decoder: any Decoder) throws {
		let container = try decoder.singleValueContainer()
		let anyDecodable = try container.decode(AnyDecodable.self)
		let jsonData = try JSONSerialization.data(withJSONObject: anyDecodable.value)
		try self.init(jsonData: jsonData)
	}
}
