import Foundation
import SargonUniFFI

extension SargonBuildInformation: SargonModel {}

extension SargonBuildInformation {
    public static func get() -> Self {
        buildInformation()
    }
}
