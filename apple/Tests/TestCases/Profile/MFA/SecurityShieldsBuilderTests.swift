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

	@Test("Complete")
	func complete() throws {
		let builder = SecurityShieldBuilder()
		builder.setName(name: "S.H.I.E.L.D.")
		builder.numberOfDaysUntilAutoConfirm = 42

		#expect(builder.validate() == .PrimaryRoleMustHaveAtLeastOneFactor)

		// Primary
		builder.addFactorSourceToPrimaryThreshold(factorSourceId: .sampleDevice)
		builder.threshold = 1
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
