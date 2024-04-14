//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension Personas {
	public init(_ elements: [Persona]) {
		self = newPersonas(personas: elements)
	}
	
	public var elements: [Persona] {
		getPersonas(personas: self)
	}
}
