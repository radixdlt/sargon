//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension LedgerHardwareWalletModel {
    public init(string: String) throws {
        self = try newLedgerHwWalletModelFromString(string: string)
    }
    
    public func toString() -> String {
        ledgerHwWalletModelToString(model: self)
    }
}
