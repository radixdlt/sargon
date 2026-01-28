import Foundation
import SargonUniFFI

public typealias FactorSourceID = FactorSourceId

// MARK: - FactorSourceID + SargonModel
extension FactorSourceID: SargonModel {}

// MARK: - FactorSourceID + CustomStringConvertible
extension FactorSourceID: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

// MARK: - FactorSourceID + FactorSourceIDProtocol
extension FactorSourceID: FactorSourceIDProtocol {
	public var asGeneral: FactorSourceID {
		self
	}
}

extension FactorSourceID {
	public func extract<F: FactorSourceIDSpecificProtocol>(_ type: F.Type = F.self) -> F? {
		F.extract(from: self)
	}

	public func extract<F: FactorSourceIDSpecificProtocol>(as _: F.Type = F.self) throws -> F {
		guard let extracted = extract(F.self) else {
			throw IncorrectFactorSourceIDType()
		}
		return extracted
	}
}

// MARK: - IncorrectFactorSourceIDType
public struct IncorrectFactorSourceIDType: Swift.Error {}
