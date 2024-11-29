import Foundation
import SargonUniFFI

// MARK: - EventKind + SargonModel
extension EventKind: SargonModel {
	public static let sample: Self = .accountAdded
	public static let sampleOther: Self = .accountUpdated
}

// MARK: - EventKind + CaseIterable
extension EventKind: CaseIterable {}
