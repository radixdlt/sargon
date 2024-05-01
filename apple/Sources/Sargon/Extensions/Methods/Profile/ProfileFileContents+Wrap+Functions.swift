//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-01.
//

import Foundation
import SargonUniFFI

extension ProfileFileContents {
	public func hash(into hasher: inout Hasher) {
		hasher.combine(profileFileContentsHashValue(contents: self))
	}
}

extension ProfileFileContents {
	public static func == (lhs: Self, rhs: Self) -> Bool {
		profileFileContentsEquals(lhs: rhs, rhs: rhs)
	}
}
