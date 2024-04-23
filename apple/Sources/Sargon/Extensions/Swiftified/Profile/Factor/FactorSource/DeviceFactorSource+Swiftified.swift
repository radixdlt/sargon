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

extension DeviceFactorSource: FactorSourceProtocol {
	
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
