//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-21.
//

import Foundation
import SargonUniFFI

// MARK: - BasePersonaDataEntryProtocol
public protocol BasePersonaDataEntryProtocol {
	func embed() -> PersonaData.Entry
}
