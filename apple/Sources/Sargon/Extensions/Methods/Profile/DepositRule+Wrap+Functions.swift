//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

// DepositRule -> SargonStringCodable
extension DepositRule {
    public init(jsonStringLiteral: String) throws {
        self = try newDepositRuleFromJsonString(jsonString: jsonStringLiteral)
    }
    
    public func jsonStringLiteral() -> String {
        depositRuleToJsonString(rule: self)
    }
}
