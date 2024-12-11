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
		let builder = SecurityShieldBuilder()
		#expect(builder.name == "My Shield")
		builder.name = "S.H.I.E.L.D"
		#expect(builder.name == "S.H.I.E.L.D")
	}

	@Test("threshold")
	func threshold() {
		let builder = SecurityShieldBuilder()
		#expect(builder.threshold == 0)
		builder.setThreshold(threshold: 42)
		#expect(builder.threshold == 42)
	}

	@Test("days")
	func days() {
		let builder = SecurityShieldBuilder()
		#expect(builder.numberOfDaysUntilAutoConfirm == 14)
		builder.setNumberOfDaysUntilAutoConfirm(numberOfDays: 237)
		#expect(builder.numberOfDaysUntilAutoConfirm == 237)
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
		let builder = SecurityShieldBuilder()
		let x: FactorSourceID = .sampleDevice
		let y: FactorSourceID = .sampleLedger
		let z: FactorSourceID = .sampleArculus
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: x)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: y)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: z)
		builder.threshold = 3

		builder.addFactorSourceToRecoveryOverride(factorSourceId: y)
		#expect(builder.recoveryRoleFactors == [y])

		#expect(builder.threshold == 3)

		builder.removeFactorFromPrimary(factorSourceId: x)
		#expect(builder.threshold == 2)

		builder.removeFactorFromAllRoles(factorSourceId: y)
		#expect(builder.recoveryRoleFactors == []) // assert `y` is removed from Recovery and Primary
		#expect(builder.threshold == 1)

		builder.removeFactorFromPrimary(factorSourceId: z)
		#expect(builder.threshold == 0)
		#expect(builder.primaryRoleThresholdFactors == [])
	}

	@Test("basic validation")
	func basicValidation() throws {
		let builder = SecurityShieldBuilder()
		#expect(builder.validate() == .PrimaryRoleMustHaveAtLeastOneFactor)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice) // did not get added, duplicates are not allowed
		#expect(builder.primaryRoleThresholdFactors == [.sampleDevice])
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDeviceOther)

		#expect(builder.validate() == .RecoveryRoleMustHaveAtLeastOneFactor)
		builder.removeFactorFromPrimary(factorSourceId: .sampleDeviceOther)
		builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedger)

		#expect(builder.validate() == .ConfirmationRoleMustHaveAtLeastOneFactor)
		builder.addFactorSourceToConfirmationOverride(factorSourceId: .sampleArculus)
		#expect(builder.validate() == nil)
		#expect((try? builder.build()) != nil)
	}

	@Test("primary role with threshold factors cannot have a threshold value of zero")
	func primaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero() throws {
		let builder = SecurityShieldBuilder()
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleLedger)
		builder.threshold = 0
		#expect(builder.validate() == .PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero)
	}

	@Test("cannot add forbidden FactorSourceKinds")
	func preventAddOfForbiddenFactorSourceKinds() throws {
		let builder = SecurityShieldBuilder()

		// Primary
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleTrustedContact) // Verboten
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleSecurityQuestions) // Verboten

		// Recovery
		builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleSecurityQuestions) // Verboten
		builder.addFactorSourceToRecoveryOverride(factorSourceId: .samplePassword) // Verboten

		// Confirmation
		builder.addFactorSourceToConfirmationOverride(factorSourceId: .sampleTrustedContact) // Verboten

		#expect(builder.primaryRoleThresholdFactors.isEmpty)
		#expect(builder.recoveryRoleFactors.isEmpty)
		#expect(builder.confirmationRoleFactors.isEmpty)
	}

	@Test("Primary can only contain one DeviceFactorSource")
	func primaryCanOnlyContainOneDeviceFactorSourceThreshold() throws {
		let builder = SecurityShieldBuilder()
		let factor = FactorSourceId.sampleDevice
		let other = FactorSourceId.sampleDeviceOther
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: factor)
		builder.addFactorSourceToPrimaryOverride(factorSourceId: other)
		#expect(builder.primaryRoleThresholdFactors == [factor])
		#expect(builder.primaryRoleOverrideFactors == [])

		builder.removeFactorFromPrimary(factorSourceId: factor)

		builder.addFactorSourceToPrimaryOverride(factorSourceId: factor)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: other)
		#expect(builder.primaryRoleThresholdFactors == [])
		#expect(builder.primaryRoleOverrideFactors == [factor])
	}

	@Test("Primary password never alone")
	func primaryPasswordNeverAlone() {
		let builder = SecurityShieldBuilder()
		builder.addFactorSourceToPrimaryOverride(factorSourceId: .samplePassword) // not allowed
		#expect(builder.primaryRoleOverrideFactors.isEmpty)

		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .samplePassword)
		#expect(builder.validate() == .PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero)
		builder.threshold = 0
		#expect(builder.validate() == .PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero)
		builder.threshold = 1
		#expect(builder.validate() == .PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleLedger)
		#expect(builder.validate() == .PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne)
		builder.threshold = 2

		builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleArculus)
		builder.addFactorSourceToConfirmationOverride(factorSourceId: .sampleArculusOther)

		let shield = try! builder.build()

		#expect(shield.matrixOfFactors.primaryRole.overrideFactors.isEmpty)
		#expect(shield.matrixOfFactors.primaryRole.threshold == 2)
		#expect(shield.matrixOfFactors.primaryRole.thresholdFactors == [.samplePassword, .sampleLedger])
	}

	@Test("Build")
	func build() throws {
		let builder = SecurityShieldBuilder()
		builder.setName(name: "S.H.I.E.L.D.")
		builder.numberOfDaysUntilAutoConfirm = 42

		#expect(builder.validate() == .PrimaryRoleMustHaveAtLeastOneFactor)

		// Primary
		#expect(builder.threshold == 0)
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice) // bumps threshold
		#expect(builder.threshold == 1)
		builder.addFactorSourceToPrimaryOverride(factorSourceId: .sampleArculus)
		builder.addFactorSourceToPrimaryOverride(factorSourceId: .sampleArculusOther)

		// Recovery
		builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedger)
		builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedgerOther)

		// Confirmation
		builder.addFactorSourceToConfirmationOverride(factorSourceId: .sampleDevice)

		builder.removeFactorFromPrimary(factorSourceId: .sampleArculusOther)
		builder.removeFactorFromRecovery(factorSourceId: .sampleLedgerOther)

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

	@Test("selected factor sources for role status")
	func selectedFactorSourcesForRoleStatus() {
		let builder = SecurityShieldBuilder()
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .samplePassword)
		builder.addFactorSourceToRecoveryOverride(factorSourceId: .sampleLedger)

		#expect(builder.selectedFactorSourcesForRoleStatus(role: .primary) == .invalid)
		#expect(builder.selectedFactorSourcesForRoleStatus(role: .recovery) == .optimal)
		#expect(builder.selectedFactorSourcesForRoleStatus(role: .confirmation) == .insufficient)
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
