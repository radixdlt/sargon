//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension OffDeviceMnemonicFactorSource: SargonModel {}
extension OffDeviceMnemonicFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

extension OffDeviceMnemonicFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .offDeviceMnemonic
	
	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .offDeviceMnemonic(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}
	
	public var asGeneral: FactorSource {
		.offDeviceMnemonic(value: self)
	}
	
	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}
	
	public var factorSourceKind: FactorSourceKind {
		.offDeviceMnemonic
	}
	
	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
