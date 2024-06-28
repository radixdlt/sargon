//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-26.
//

import Foundation
import Sargon
import ComposableArchitecture

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static var savedGateways: Self {
		Self.sharedSavedGateways
	}
}

extension PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static let sharedSavedGateways = Self(
		SargonKey(
			accessing: \.gateways,
			fetchIf: \.affectsSavedGateways
		),
			.default
	)
	
}
