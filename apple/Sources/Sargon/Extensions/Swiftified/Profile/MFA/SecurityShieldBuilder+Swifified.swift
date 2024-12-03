import SargonUniFFI

extension FactorSourceValidationStatus {
	public var factorSourceID: FactorSourceID {
		factorSourceId()
	}

//	public var reasonIfInvalid: SecurityShieldBuilderInvalidReason? {
//		self.reasonIfInvalid()
//	}

	public var role: RoleKind {
		role()
	}
}

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
		getPrimaryThreshold()
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
