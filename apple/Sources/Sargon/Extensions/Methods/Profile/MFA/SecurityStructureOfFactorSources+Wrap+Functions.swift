import Foundation
import SargonUniFFI

extension SecurityStructureOfFactorSources {
    public init(
        metadata: SecurityStructureMetadata,
        numberOfDaysUntilAutoConfirmation: UInt16,
        matrixOfFactors: MatrixOfFactorSources
    ) {
        self = newSecurityStructureOfFactorSourcesAutoInDays(
            metadata: metadata,
            numberOfDaysUntilAutoConfirmation: numberOfDaysUntilAutoConfirmation,
            matrixOfFactors: matrixOfFactors
        )
    }
}
