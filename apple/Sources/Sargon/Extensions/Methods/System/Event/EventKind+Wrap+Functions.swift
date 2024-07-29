//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-24.
//

import Foundation
import SargonUniFFI

extension EventKind {
	public static let allCases: [Self] = eventKindAll()
	
	public var affectsCurrentAccounts: Bool {
		eventKindAffectsCurrentAccounts(eventKind: self)
	}
	
	public var affectsCurrentNetwork: Bool {
		eventKindAffectsCurrentNetwork(eventKind: self)
	}
	
	public var affectsSavedGateways: Bool {
		eventKindAffectsSavedGateways(eventKind: self)
	}
}
