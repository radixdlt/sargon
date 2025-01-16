import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import Testing

// MARK: - ShieldTests
@Suite("ShieldBuilder")
struct ShieldTests {
	@Test("name")
	func name() {
		var builder = SecurityShieldBuilder()
		#expect(builder.name == "My Shield")
		builder = builder.setName(name: "S.H.I.E.L.D")
		#expect(builder.name == "S.H.I.E.L.D")
	}

	@Test("threshold")
	func threshold() {
		var builder = SecurityShieldBuilder()
		#expect(builder.threshold == Threshold.all)
		builder = builder.setThreshold(threshold: Threshold.specific(42))
		#expect(builder.threshold == Threshold.specific(42))
	}

	@Test("days")
	func days() {
		var builder = SecurityShieldBuilder()
		#expect(builder.timePeriodUntilAutoConfirm == TimePeriod(days: 14))
		builder = builder.setTimePeriodUntilAutoConfirm(timePeriod: TimePeriod(days: 237))
		#expect(builder.timePeriodUntilAutoConfirm == TimePeriod(days: 237))
	}

	@Test("empty primary threshold")
	func emptyThresholdFactors() {
		let builder = SecurityShieldBuilder()
		#expect(builder.primaryRoleThresholdFactors == [])
	}

	@Test("empty primary override")
	func emptyOverrideFactors() {
		let builder = SecurityShieldBuilder()
		#expect(builder.primaryRoleOverrideFactors == [])
	}

	@Test("empty recovery")
	func emptyRecoveryFactors() {
		let builder = SecurityShieldBuilder()
		#expect(builder.recoveryRoleFactors == [])
	}

	@Test("empty confirmation")
	func emptyConfirmationFactors() {
		let builder = SecurityShieldBuilder()
		#expect(builder.confirmationRoleFactors == [])
	}

	@Test("primary override validation status trustedContact")
	func primValidationStatusTrustedContact() {
		let builder = SecurityShieldBuilder()
		#expect(builder.validationForAdditionOfFactorSourceToPrimaryOverrideForEach(factorSources: [TrustedContactFactorSource.sample.asGeneral.id]).compactMap(\.reasonIfInvalid) == [FactorSourceValidationStatusReasonIfInvalid.nonBasic(SecurityShieldBuilderInvalidReason.PrimaryCannotContainTrustedContact)])
	}

	@Test("Auto lowering of threshold upon deletion")
	func deleteFactorSourceFromPrimaryLowersThreshold() {
		let x: FactorSourceID = .sampleDevice
		let y: FactorSourceID = .sampleLedger
		let z: FactorSourceID = .sampleArculus
		var builder = SecurityShieldBuilder()
			.addFactorSourceToPrimaryThreshold(factorSourceId: x)
			.addFactorSourceToPrimaryThreshold(factorSourceId: y)
			.addFactorSourceToPrimaryThreshold(factorSourceId: z)
			.setThreshold(threshold: Threshold.specific(3))
			.addFactorSourceToRecoveryOverride(factorSourceId: y)
		#expect(builder.recoveryRoleFactors == [y])

		#expect(builder.threshold == Threshold.specific(3))

		builder = builder.removeFactorFromPrimary(factorSourceId: x, factorListKind: FactorListKind.threshold)
		#expect(builder.threshold == Threshold.specific(2))

		builder = builder.removeFactorFromAllRoles(factorSourceId: y)
		#expect(builder.recoveryRoleFactors == []) // assert `y` is removed from Recovery and Primary
		#expect(builder.threshold == Threshold.specific(1))

		builder = builder.removeFactorFromPrimary(factorSourceId: z, factorListKind: FactorListKind.threshold)
		#expect(builder.threshold == Threshold.all)
		#expect(builder.primaryRoleThresholdFactors == [])
	}

	@Test("basic validation")
	func basicValidation() throws {
		var builder = SecurityShieldBuilder()
		#expect(builder.validate() == .MissingAuthSigningFactor)
		builder = builder.setAuthenticationSigningFactor(new: .sampleDevice)
		#expect(builder.validate() == .PrimaryRoleMustHaveAtLeastOneFactor)
		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice)
			.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice) // did not get added, duplicates are not allowed
		#expect(builder.primaryRoleThresholdFactors == [.sampleDevice])

		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDeviceOther) // actually this is added
		#expect(builder.validate() == .PrimaryCannotHaveMultipleDevices)
		builder = builder.removeFactorFromPrimary(factorSourceId: .sampleDeviceOther, factorListKind: FactorListKind.threshold)

		#expect(builder.validate() == .RecoveryRoleMustHaveAtLeastOneFactor)
		builder = builder.removeFactorFromPrimary(factorSourceId: .sampleDeviceOther, factorListKind: FactorListKind.threshold)
			.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedger)

		#expect(builder.validate() == .ConfirmationRoleMustHaveAtLeastOneFactor)
		builder = builder.addFactorSourceToConfirmationOverride(factorSourceId: .sampleArculus)
			.setAuthenticationSigningFactor(new: .sampleDevice)

		#expect(builder.validate() == nil)
		#expect((try? builder.build()) != nil)
	}

	@Test("cannot add forbidden FactorSourceKinds")
	func preventAddOfForbiddenFactorSourceKinds() throws {
		let builder = SecurityShieldBuilder()
			// Primary
			.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleTrustedContact) // Verboten
			.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleSecurityQuestions) // Verboten
			// Recovery
			.addFactorSourceToRecoveryOverride(factorSourceId: .sampleSecurityQuestions) // Verboten
			.addFactorSourceToRecoveryOverride(factorSourceId: .samplePassword) // Verboten
			// Confirmation
			.addFactorSourceToConfirmationOverride(factorSourceId: .sampleTrustedContact) // Verboten

		#expect(builder.primaryRoleThresholdFactors.isEmpty)
		#expect(builder.recoveryRoleFactors.isEmpty)
		#expect(builder.confirmationRoleFactors.isEmpty)
	}

	@Test("Primary can contain two DeviceFactorSource while building - but is never valid")
	func primaryCanOnlyContainOneDeviceFactorSourceThreshold() throws {
		let factor = FactorSourceId.sampleDevice
		let other = FactorSourceId.sampleDeviceOther
		var builder = SecurityShieldBuilder()
			.addFactorSourceToPrimaryThreshold(factorSourceId: factor)
			.addFactorSourceToPrimaryOverride(factorSourceId: other)
		#expect(builder.primaryRoleThresholdFactors == [factor])
		#expect(builder.primaryRoleOverrideFactors == [other])

		builder = builder.removeFactorFromPrimary(factorSourceId: factor, factorListKind: FactorListKind.threshold)
			.addFactorSourceToPrimaryOverride(factorSourceId: factor)
			.addFactorSourceToPrimaryThreshold(factorSourceId: other)
		#expect(builder.primaryRoleThresholdFactors == [other])
		#expect(builder.primaryRoleOverrideFactors == [other, factor])

		// But when validated/built is err
		#expect(builder.validate() != nil)
		#expect((try? builder.build()) == nil)
	}

	@Test("Primary password never alone")
	func primaryPasswordNeverAlone() {
		var builder = SecurityShieldBuilder()
			.setAuthenticationSigningFactor(new: .sampleDevice)
			.addFactorSourceToPrimaryOverride(factorSourceId: .samplePassword) // not allowed
		#expect(builder.primaryRoleOverrideFactors.isEmpty)

		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .samplePassword)
		#expect(builder.validate() == .PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor)
		builder = builder.setThreshold(threshold: Threshold.all)

		builder = builder.setThreshold(threshold: Threshold.specific(1))
		#expect(builder.validate() == .PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor)
		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleLedger)
		#expect(builder.validate() == .PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne)
		builder = builder.setThreshold(threshold: Threshold.specific(2))

		builder = builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleArculus)
			.addFactorSourceToConfirmationOverride(factorSourceId: .sampleArculusOther)

		builder.setAuthenticationSigningFactor(new: .sampleDevice)

		let shield = try! builder.build()

		#expect(shield.matrixOfFactors.primaryRole.overrideFactors.isEmpty)
		#expect(shield.matrixOfFactors.primaryRole.threshold == Threshold.specific(2))
		#expect(shield.matrixOfFactors.primaryRole.thresholdFactors == [.samplePassword, .sampleLedger])
	}

	@Test("Build")
	func build() throws {
		var builder = SecurityShieldBuilder()
			.setName(name: "S.H.I.E.L.D.")
			.setTimePeriodUntilAutoConfirm(timePeriod: TimePeriod(days: 42))

		#expect(builder.validate() == .MissingAuthSigningFactor)
		builder = builder.setAuthenticationSigningFactor(new: .sampleDevice)
		#expect(builder.validate() == .PrimaryRoleMustHaveAtLeastOneFactor)

		// Primary
		#expect(builder.threshold == Threshold.all)
		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice)
		#expect(builder.threshold == Threshold.all)
		builder = builder.addFactorSourceToPrimaryOverride(factorSourceId: .sampleArculus)
			.addFactorSourceToPrimaryOverride(factorSourceId: .sampleArculusOther)
			// Recovery
			.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedger)
			.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedgerOther)
			// Confirmation
			.addFactorSourceToConfirmationOverride(factorSourceId: .sampleDevice)
			// Remove
			.removeFactorFromPrimary(factorSourceId: .sampleArculusOther, factorListKind: FactorListKind.override)
			.removeFactorFromRecovery(factorSourceId: .sampleLedgerOther)

		builder.setAuthenticationSigningFactor(new: .sampleDevice)

		// Validate
		#expect(builder.validate() == nil)

		// Build
		let shield0 = try builder.build()
		let shield = try builder.build()
		#expect(shield0 == shield)

		// Assert
		#expect(shield.metadata.displayName == "S.H.I.E.L.D.")
		#expect(shield.matrixOfFactors.primaryRole.overrideFactors == [.sampleArculus])
		#expect(shield.matrixOfFactors.primaryRole.thresholdFactors == [.sampleDevice])

		#expect(shield.matrixOfFactors.recoveryRole.overrideFactors == [.sampleLedger])
		#expect(shield.matrixOfFactors.recoveryRole.thresholdFactors == [])

		#expect(shield.matrixOfFactors.confirmationRole.overrideFactors == [.sampleDevice])
		#expect(shield.matrixOfFactors.confirmationRole.thresholdFactors == [])
	}

	@Test("selected primary threshold factors status")
	func selectedPrimaryThresholdFactorsStatusInvalid() {
		let builder = SecurityShieldBuilder()
			.addFactorSourceToPrimaryThreshold(factorSourceId: .samplePassword)

		#expect(builder.selectedPrimaryThresholdFactorsStatus() == .invalid(reason: SelectedPrimaryThresholdFactorsStatusInvalidReason.cannotBeUsedAlone(factorSourceKind: FactorSourceKind.password)))
	}

	@Test("selected primary threshold factors status")
	func selectedPrimaryThresholdFactorsStatus() {
		var builder = SecurityShieldBuilder()

		#expect(builder.selectedPrimaryThresholdFactorsStatus() == .insufficient)

		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice)

		#expect(builder.selectedPrimaryThresholdFactorsStatus() == .suboptimal)

		builder = builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleLedger)

		#expect(builder.selectedPrimaryThresholdFactorsStatus() == .optimal)
	}
}

#if DEBUG
extension FactorSourceID {
	public static let sampleDevice = DeviceFactorSource.sample.asGeneral.id
	public static let sampleDeviceOther = DeviceFactorSource.sampleOther.asGeneral.id

	public static let sampleLedger = LedgerHardwareWalletFactorSource.sample.asGeneral.id
	public static let sampleLedgerOther = LedgerHardwareWalletFactorSource.sampleOther.asGeneral.id

	public static let sampleArculus = ArculusCardFactorSource.sample.asGeneral.id
	public static let sampleArculusOther = ArculusCardFactorSource.sampleOther.asGeneral.id

	public static let samplePassword = PasswordFactorSource.sample.asGeneral.id
	public static let samplePasswordOther = PasswordFactorSource.sampleOther.asGeneral.id

	public static let sampleOffDeviceMnemonic = OffDeviceMnemonicFactorSource.sample.asGeneral.id
	public static let sampleOffDeviceMnemonicOther = OffDeviceMnemonicFactorSource.sampleOther.asGeneral.id

	public static let sampleTrustedContact = TrustedContactFactorSource.sample.asGeneral.id
	public static let sampleTrustedContactOther = TrustedContactFactorSource.sampleOther.asGeneral.id

	public static let sampleSecurityQuestions = SecurityQuestionsNotProductionReadyFactorSource.sample.asGeneral.id
	public static let sampleSecurityQuestionsOther = SecurityQuestionsNotProductionReadyFactorSource.sampleOther.asGeneral.id
}
#endif
