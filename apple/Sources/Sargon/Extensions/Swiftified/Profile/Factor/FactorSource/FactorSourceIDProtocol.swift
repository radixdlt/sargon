//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-24.
//

import Foundation

public protocol FactorSourceIDProtocol: SargonModel & CustomStringConvertible {
	var asGeneral: FactorSourceID { get }
	func toString() -> String
}

extension FactorSourceIDProtocol {
	public var description: String {
		toString()
	}
}

public protocol FactorSourceIDSpecificProtocol: FactorSourceIDProtocol & Codable {
	static func extract(from someFactorSourceID: some FactorSourceIDProtocol) -> Self?
}
