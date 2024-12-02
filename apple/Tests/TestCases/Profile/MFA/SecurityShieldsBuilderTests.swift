import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import Testing

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
		#expect(throws: CommonError.RoleMustHaveAtLeastOneFactor) {
			try builder.setThreshold(threshold: 0)
		}
	}

	@Test("days")
	func days() {
		let builder = SecurityShieldBuilder()
		#expect(builder.numberOfDaysUntilAutoConfirm == 14)
		#expect(throws: CommonError.NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero) {
			try builder.setNumberOfDaysUntilAutoConfirm(numberOfDays: 0)
		}
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
	func primValidationStatusTrustedContact() throws {
		let builder = SecurityShieldBuilder()
		try #expect(builder.validationForAdditionOfFactorSourceToPrimaryOverrideForEach(factorSources: [TrustedContactFactorSource.sample.asGeneral.id]).map(\.validationError) == [CommonError.PrimaryCannotContainTrustedContact])
	}
}
