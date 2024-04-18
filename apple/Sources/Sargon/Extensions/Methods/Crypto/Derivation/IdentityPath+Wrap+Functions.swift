//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-18.
//

import Foundation
import SargonUniFFI

extension IdentityPath {
	public init(networkID: NetworkID, keyKind: Cap26KeyKind, index: HDPathValue) {
		self = newIdentityPath(networkId: networkID, keyKind: keyKind, index: index)
	}
}
