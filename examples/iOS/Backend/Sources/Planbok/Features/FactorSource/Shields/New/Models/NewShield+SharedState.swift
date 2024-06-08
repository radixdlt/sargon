//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture

public struct ThresholdsForRoles: Hashable, Sendable {
	private var primary: FactorThreshold = .any
	private var recovery: FactorThreshold = .any
	private var confirmation: FactorThreshold = .any
	public subscript(role: Role) -> FactorThreshold {
		get {
			switch role {
			case .primary: return self.primary
			case .recovery: return self.recovery
			case .confirmation: return self.confirmation
			}
		}
		set {
			switch role {
			case .primary: self.primary = newValue
			case .recovery: self.recovery = newValue
			case .confirmation: self.confirmation = newValue
			}
		}
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<InMemoryKey<ThresholdsForRoles>> {
	static var thresholds: Self {
		PersistenceKeyDefault(
			.inMemory("thresholds"),
			ThresholdsForRoles()
		)
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<InMemoryKey<Factor?>> {
	static var pickedFactor: Self {
		PersistenceKeyDefault(
			.inMemory("pickedFactorSource"),
			nil
		)
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<InMemoryKey<Factors>> {
	static var thresholdFactors: Self {
		PersistenceKeyDefault(
			.inMemory("thresholdFactors"),
			[]
		)
	}
}
extension PersistenceReaderKey where Self == PersistenceKeyDefault<InMemoryKey<Factors>> {
	static var overrideFactors: Self {
		PersistenceKeyDefault(
			.inMemory("overrideFactors"),
			[]
		)
	}
}

extension Observable {
	
	/// Uh, this is hacky!
	public func pickedPendingFactors() -> FactorSources {
        @Shared(.thresholdFactors) var thresholdFactors = [Factor(factorSource: .sample)]
		@Shared(.overrideFactors) var overrideFactors = []
		
		var picked = FactorSources()
		func addFrom(_ factors: Factors?) {
			guard let factors else { return  }
			picked.append(contentsOf: factors.compactMap(\.factorSource))
		}
		addFrom(thresholdFactors)
		addFrom(overrideFactors)
		
		return picked
	}
	
	/// Uh, this is hacky!
	public func idsOfAllPicked() -> Set<FactorSource.ID> {
		let allPicked = pickedPendingFactors()
		return Set(allPicked.map(\.id))
	}
}


