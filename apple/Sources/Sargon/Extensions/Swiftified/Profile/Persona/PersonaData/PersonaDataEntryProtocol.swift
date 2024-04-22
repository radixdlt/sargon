//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-21.
//

import Foundation

// MARK: - PersonaDataEntryProtocol
public protocol PersonaDataEntryProtocol: BasePersonaDataEntryProtocol & SargonModel & Codable & CustomStringConvertible {
	static var kind: PersonaData.Entry.Kind { get }
	func embed() -> PersonaData.Entry
	static func extract(from entry: PersonaData.Entry) -> Self?
}

extension PersonaDataEntryProtocol {
	public var kind: PersonaData.Entry.Kind { Self.kind }
}
