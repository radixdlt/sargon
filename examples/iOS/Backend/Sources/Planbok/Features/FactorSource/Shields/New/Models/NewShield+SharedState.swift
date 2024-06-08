//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture

public struct NewShieldDraft: Hashable, Sendable {
	public struct MatrixOfFactorsForRole: Hashable, Sendable {
		public var thresholdFactors: Factors
		public var threshold: FactorThreshold
		public var overrideFactors: Factors
		fileprivate init(thresholdFactors: Factors = [], threshold: FactorThreshold = .any, overrideFactors: Factors = []) {
			self.threshold = threshold
			self.thresholdFactors = thresholdFactors
			self.overrideFactors = overrideFactors
		}
		var usedFactorSources: FactorSources {
			var all: FactorSources = []
			all.append(contentsOf: thresholdFactors.compactMap(\.factorSource))
			all.append(contentsOf: overrideFactors.compactMap(\.factorSource))
			return all
		}
	}
	public var numberOfDaysUntilAutoConfirmation: UInt16 = 14
	private var primary: MatrixOfFactorsForRole
	private var recovery: MatrixOfFactorsForRole
	private var confirmation: MatrixOfFactorsForRole

	public var usedFactorSources: FactorSources {
		var allUsed: FactorSources = []
		allUsed.append(contentsOf: primary.usedFactorSources)
		allUsed.append(contentsOf: recovery.usedFactorSources)
		allUsed.append(contentsOf: confirmation.usedFactorSources)
		return allUsed
	}
	public var pendingFactor: Factor?
	
	public init() {
		self.primary = MatrixOfFactorsForRole()
		self.recovery = MatrixOfFactorsForRole()
		self.confirmation = MatrixOfFactorsForRole()
	}

	public subscript(role: Role) -> MatrixOfFactorsForRole {
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

extension PersistenceReaderKey where Self == PersistenceKeyDefault<InMemoryKey<NewShieldDraft>> {
	static var newShieldDraft: Self {
		PersistenceKeyDefault(
			.inMemory("newShieldDraft"),
			NewShieldDraft()
		)
	}
}


//extension Observable {
//	
//	/// Uh, this is hacky!
//	public func pickedPendingFactors() -> FactorSources {
//        @Shared(.thresholdFactors) var thresholdFactors = [Factor(factorSource: .sample)]
//		@Shared(.overrideFactors) var overrideFactors = []
//		
//		var picked = FactorSources()
//		func addFrom(_ factors: Factors?) {
//			guard let factors else { return  }
//			picked.append(contentsOf: factors.compactMap(\.factorSource))
//		}
//		addFrom(thresholdFactors)
//		addFrom(overrideFactors)
//		
//		return picked
//	}
//	
//	/// Uh, this is hacky!
//	public func idsOfAllPicked() -> Set<FactorSource.ID> {
//		let allPicked = pickedPendingFactors()
//		return Set(allPicked.map(\.id))
//	}
//}


