//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation

extension LedgerHardwareWalletModel: SargonModel {}
extension LedgerHardwareWalletModel: CustomStringConvertible {
    public var description: String {
        toString()
    }
}
extension LedgerHardwareWalletModel {
    public var rawValue: String {
        toString()
    }
}
