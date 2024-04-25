//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension PrivateHierarchicalDeterministicFactorSource: SargonModel {}
extension PrivateHierarchicalDeterministicFactorSource: BaseFactorSourceProtocol {
	
	public var common: FactorSourceCommon {
		get { factorSource.common }
		set {
			factorSource.common = newValue
		}
	}
	
	public var factorSourceID: FactorSourceID {
		factorSource.factorSourceID
	}
	
	public var factorSourceKind: FactorSourceKind {
		factorSource.factorSourceKind
	}
	
	public var asGeneral: FactorSource {
		factorSource.asGeneral
	}
	
	public var supportsOlympia: Bool {
		factorSource.supportsOlympia
	}
	
	public var supportsBabylon: Bool {
		factorSource.supportsBabylon
	}
	
	
}
