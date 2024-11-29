import Foundation
import SargonUniFFI

// MARK: - SargonBuildInformation + SargonModel
extension SargonBuildInformation: SargonModel {}

extension SargonBuildInformation {
	public static func get() -> Self {
		buildInformation()
	}
}
