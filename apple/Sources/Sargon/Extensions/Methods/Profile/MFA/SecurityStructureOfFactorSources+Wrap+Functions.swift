import Foundation
import SargonUniFFI

extension SecurityStructureOfFactorSources {
	public init(
		metadata: SecurityStructureMetadata,
		numberOfDaysUntilAutoConfirmation: UInt16,
		matrixOfFactors: MatrixOfFactorSources
	) {
		assert(matrixOfFactors.primaryRole.thresholdFactors.count >= matrixOfFactors.primaryRole.threshold)
		assert(matrixOfFactors.recoveryRole.thresholdFactors.count >= matrixOfFactors.recoveryRole.threshold)
		assert(matrixOfFactors.confirmationRole.thresholdFactors.count >= matrixOfFactors.confirmationRole.threshold)
		self = newSecurityStructureOfFactorSourcesAutoInDays(
			metadata: metadata,
			numberOfDaysUntilAutoConfirmation: numberOfDaysUntilAutoConfirmation,
			matrixOfFactors: matrixOfFactors
		)
	}
}
