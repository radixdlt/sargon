//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension LedgerHardwareWalletFactorSource: SargonModel {}

extension LedgerHardwareWalletFactorSource: FactorSourceProtocol {
	
	public var asGeneral: FactorSource {
		.ledger(value: self)
	}
	
	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}
	
	public var factorSourceKind: FactorSourceKind {
		.ledgerHqHardwareWallet
	}
	
	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
