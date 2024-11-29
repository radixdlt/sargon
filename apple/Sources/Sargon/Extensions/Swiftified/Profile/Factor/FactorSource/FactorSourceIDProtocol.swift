import Foundation

// MARK: - FactorSourceIDProtocol
public protocol FactorSourceIDProtocol: SargonModel & CustomStringConvertible {
	var asGeneral: FactorSourceID { get }
	func toString() -> String
}

extension FactorSourceIDProtocol {
	public var description: String {
		toString()
	}
}

// MARK: - FactorSourceIDSpecificProtocol
public protocol FactorSourceIDSpecificProtocol: FactorSourceIDProtocol & Codable {
	static func extract(from someFactorSourceID: some FactorSourceIDProtocol) -> Self?
}
