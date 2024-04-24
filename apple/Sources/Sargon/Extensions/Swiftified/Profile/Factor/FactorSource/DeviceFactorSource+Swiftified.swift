//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension DeviceFactorSource: SargonModel {}

extension DeviceFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

extension DeviceFactorSource: FactorSourceSpecificProtocol {
	public static let kind: FactorSourceKind = .device
	
	public static func extract(from someFactorSource: some FactorSourceProtocol) -> Self? {
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
	
	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
