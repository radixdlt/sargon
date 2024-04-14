//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public protocol FactorSourceProtocol {
	var factorSourceID: FactorSourceID { get }
	var factorSourceKind: FactorSourceKind { get }
}
