//
//  SecurityShieldBuilder+Swiftiefied.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-12-02.
//

import SargonUniFFI

extension SecurityShieldBuilder {
	public typealias Factor = FactorSourceID
	
	/// Confirmation Role
	public var numberOfDaysUntilAutoConfirm: UInt16 {
		get { getNumberOfDaysUntilAutoConfirm() }
		set {
			precondition(newValue > 0, "Number of days until auto confirm must be greater than zero.")
			try! setNumberOfDaysUntilAutoConfirm(numberOfDays: UInt16(newValue))
		}
	}
	
	public var threshold: UInt8 {
		get {
			getPrimaryThreshold()
		}
		set {
			precondition(newValue <= primaryRoleThresholdFactors.count, "Threshold must not me greater than the number of threshold factors in the primary role.")
			try! setThreshold(threshold: newValue)
		}
	}
	
	public var primaryRoleThresholdFactors: [Factor] {
		getPrimaryThresholdFactors()
	}
	
	public var primaryRoleOverrideFactors: [Factor] {
		getPrimaryOverrideFactors()
	}
	
	public var recoveryRoleFactors: [Factor] {
		getRecoveryFactors()
	}
	
	public var confirmationRoleFactors: [Factor] {
		getConfirmationFactors()
	}
	
	/// Name of the shield
	public var name: String {
		get {
			getName()
		}
		set {
			setName(name: newValue)
		}
	}
	
}
