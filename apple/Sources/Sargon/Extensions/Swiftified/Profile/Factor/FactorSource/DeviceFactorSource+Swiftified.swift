import Foundation
import SargonUniFFI

// MARK: - DeviceFactorSource + SargonModel
extension DeviceFactorSource: SargonModel {}

// MARK: - DeviceFactorSource + Identifiable
extension DeviceFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

// MARK: - DeviceFactorSource + FactorSourceProtocol
extension DeviceFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .device

	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .device(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}

	public var asGeneral: FactorSource {
		.device(value: self)
	}

	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}

	public var factorSourceKind: FactorSourceKind {
		.device
	}

	public var supportsOlympia: Bool {
		asGeneral.supportsOlympia
	}

	public var supportsBabylon: Bool {
		asGeneral.supportsBabylon
	}
}
