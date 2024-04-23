//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension NetworkDefinition {
	public static func lookupBy(logicalName: String) throws -> Self {
		try newNetworkDefinitionLookupByName(logicalName: logicalName)
	}
}

