//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-24.
//

import Foundation
import SargonUniFFI

extension EventKind: SargonModel {
	public static let sample: Self = .accountAdded
	public static let sampleOther: Self = .accountUpdated
}
extension EventKind: CaseIterable {}

