//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-21.
//

import Foundation
import SargonUniFFI

extension PersonaDataEntryName {
	public init(jsonData: some DataProtocol) throws {
		self = try newPersonaDataEntryNameFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		personaDataEntryNameToJsonBytes(personaDataEntryName: self)
	}
}
