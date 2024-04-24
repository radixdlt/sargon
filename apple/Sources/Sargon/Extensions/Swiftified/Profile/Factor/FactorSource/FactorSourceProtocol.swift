//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public protocol FactorSourceProtocol: SargonModel {
	var factorSourceID: FactorSourceID { get }
	var factorSourceKind: FactorSourceKind { get }
	var asGeneral: FactorSource { get }
	var supportsOlympia: Bool { get }
	var supportsBabylon: Bool { get }
}

public extension FactorSourceProtocol {
	var kind: FactorSourceKind { factorSourceKind }
	var common: FactorSourceCommon {
		asGeneral.common
	}
	var cryptoParameters: FactorSourceCryptoParameters {
		common.cryptoParameters
	}
}

public protocol FactorSourceSpecificProtocol: FactorSourceProtocol {
	static var kind: FactorSourceKind { get }
	static func extract(from: some FactorSourceProtocol) -> Self?
}
