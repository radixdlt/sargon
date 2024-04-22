//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension DerivationPath: SargonModel {}
extension DerivationPath: CustomStringConvertible {
    public var description: String {
        toString()
    }
	
	/// Returns the index, non hardened, so `3H` returns `3`.
	public var nonHardenedIndex: HDPathValue {
		let component = self.path.components.last! // safe to unwrap, we disallow empty paths.
		return component.nonHardenedValue
	}
}

extension HdPathComponent {
	public var nonHardenedValue: HDPathValue {
		hdPathComponentGetNonHardenedValue(component: self)
	}
}
