import SargonUniFFI

extension SecurityShieldBuilder {
	public typealias Factor = FactorSourceID

	/// Confirmation Role
	public var timeUntilTimedConfirmationIsCallable: TimePeriod {
		getTimeUntilTimedConfirmationIsCallable()
	}

	public var threshold: Threshold {
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
		getName()
	}
}
