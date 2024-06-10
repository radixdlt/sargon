//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture

public struct MatrixOfFactorsForRole: Hashable, Sendable {
	public let role: Role
	public var thresholdFactors: Factors
	public var threshold: FactorThreshold
	public var overrideFactors: Factors
	fileprivate init(role: Role, thresholdFactors: Factors = [], threshold: FactorThreshold = .any, overrideFactors: Factors = []) {
		self.role = role
		self.threshold = threshold
		self.thresholdFactors = thresholdFactors
		self.overrideFactors = overrideFactors
	}
	init<R>(roleWithFactors: R) where R: RoleFromDraft {
		let thresholdFactors = roleWithFactors.thresholdFactors.map { Factor.init(factorSource: $0) }.asIdentified()
		self.init(
			role: R.role,
			thresholdFactors: thresholdFactors,
			threshold: .init(count: roleWithFactors.threshold, thresholdFactorsCount: thresholdFactors.count),
			overrideFactors: roleWithFactors.overrideFactors.map { Factor.init(factorSource: $0) }.asIdentified()
		)
	}
	var usedFactorSources: FactorSources {
		var all: FactorSources = []
		all.append(contentsOf: thresholdFactors.compactMap(\.factorSource))
		all.append(contentsOf: overrideFactors.compactMap(\.factorSource))
		return all
	}
	var thresholdFactorSources: [FactorSource] {
		thresholdFactors.compactMap(\.factorSource)
	}
	var overrideFactorsSources: [FactorSource] {
		overrideFactors.compactMap(\.factorSource)
	}
}

public protocol RoleFromDraft {
	
	var thresholdFactors: [FactorSource] { get }
	/**
	 * How many threshold factors that must be used to perform some function with this role.
	 */
	var threshold: UInt16 { get }
	/**
	 * Overriding / Super admin / "sudo" / God / factors, **ANY**
	 * single of these factor which can perform the function of this role,
	 * disregarding of `threshold`.
	 */
	var overrideFactors: [FactorSource] { get }
	
	static var role: Role { get }
	init(thresholdFactors: [FactorSource], threshold: UInt16, overrideFactors: [FactorSource])
	init?(draft: MatrixOfFactorsForRole)
}
extension RoleFromDraft {
	
	public init?(draft: MatrixOfFactorsForRole) {
		precondition(draft.role == Self.role)
		if draft.thresholdFactorSources.isEmpty && draft.overrideFactorsSources.isEmpty {
			return nil
		}
		if !draft.threshold.isValid(thresholdFactorCount: draft.thresholdFactorSources.count) {
			return nil
		}

		self = .init(
			thresholdFactors: draft.thresholdFactorSources,
			threshold: {
				switch draft.threshold {
				case .any: 1
				case .all: UInt16(draft.thresholdFactorSources.count)
				case let .threshold(t): t
				}
			}(),
			overrideFactors: draft.overrideFactorsSources
		)
	}
	
	
}
extension PrimaryRoleWithFactorSources: RoleFromDraft {
	public static let role: Role = .primary
}
extension RecoveryRoleWithFactorSources: RoleFromDraft {
	public static let role: Role = .recovery
}
extension ConfirmationRoleWithFactorSources: RoleFromDraft {
	public static let role: Role = .confirmation
}



public struct NewShieldDraft: Hashable, Sendable {
	public let copyOf: Shield?
	public var numberOfDaysUntilAutoConfirmation: UInt16 = 14
	private var primary: MatrixOfFactorsForRole
	private var recovery: MatrixOfFactorsForRole
	private var confirmation: MatrixOfFactorsForRole
	
	private var _primaryRole: PrimaryRoleWithFactorSources? {
		PrimaryRoleWithFactorSources(draft: primary)
	}
	private var _recoveryRole: RecoveryRoleWithFactorSources? {
		RecoveryRoleWithFactorSources(draft: recovery)
	}
	private var _confirmationRole: ConfirmationRoleWithFactorSources? {
		ConfirmationRoleWithFactorSources(draft: confirmation)
	}
	public var matrixOfFactors: MatrixOfFactorSources? {
		guard
			let primary = _primaryRole,
			let recovery = _recoveryRole,
			let confirmation = _confirmationRole
		else {
			return nil
		}
		return MatrixOfFactorSources(
			primaryRole: primary,
			recoveryRole: recovery,
			confirmationRole: confirmation
		)
	}

	public var usedFactorSources: FactorSources {
		var allUsed: FactorSources = []
		allUsed.append(contentsOf: primary.usedFactorSources)
		allUsed.append(contentsOf: recovery.usedFactorSources)
		allUsed.append(contentsOf: confirmation.usedFactorSources)
		return allUsed
	}
	public var pendingFactorID: Factor.ID?
	
	public func isValidRole(_ role: Role) -> Bool {
		switch role {
		case .confirmation: self._confirmationRole != nil
		case .recovery: self._recoveryRole != nil
		case .primary: self._primaryRole != nil
		}
	}
	
	public mutating func removeFactor(_ factor: Factor, role: Role) {
		if factor.id == pendingFactorID {
			pendingFactorID = nil // not really possible in UI, but prudent.
		}
		if self[role].thresholdFactors.contains(factor) {
			self[role].thresholdFactors.remove(factor)
			// Also decrease factor threshold if needed
			if !self[role].threshold.isValid(thresholdFactorCount: self[role].thresholdFactors.count) {
				self[role].threshold.decrease()
			}
		} else if self[role].overrideFactors.contains(factor) {
			self[role].overrideFactors.remove(factor)
		}
	}
	public mutating func pickedFactorSource(_ factorSource: FactorSource, role: Role) {
		guard let pendingFactorID else {
			assertionFailure("Expected pending...")
			return
		}
		let id = pendingFactorID
		assert(!usedFactorSources.contains(factorSource))
		if self[role].overrideFactors.contains(where: { $0.id == id }) {
			self[role].overrideFactors[id: id]?.factorSource = factorSource
		} else if self[role].thresholdFactors.contains(where: { $0.id == id }) {
			self[role].thresholdFactors[id: id]?.factorSource = factorSource
		}
		self.pendingFactorID = nil
	}
	
	public init() {
		self.copyOf = nil
		self.primary = MatrixOfFactorsForRole(role: .primary)
		self.recovery = MatrixOfFactorsForRole(role: .recovery)
		self.confirmation = MatrixOfFactorsForRole(role: .confirmation)
	}
	
	public init(copyAndEdit preset: Shield) {
		self.copyOf = preset
		self.primary = .init(roleWithFactors: preset.matrixOfFactors.primaryRole)
		self.recovery = .init(roleWithFactors: preset.matrixOfFactors.recoveryRole)
		self.confirmation = .init(roleWithFactors: preset.matrixOfFactors.confirmationRole)
		self.numberOfDaysUntilAutoConfirmation = UInt16(preset.numberOfEpochsUntilAutoConfirmation / 288)
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



