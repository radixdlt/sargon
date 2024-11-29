import Foundation
import SargonUniFFI

// MARK: - ProfileNetwork + SargonModel
extension ProfileNetwork: SargonModel {}

// MARK: - ProfileNetwork + Identifiable
extension ProfileNetwork: Identifiable {
	public typealias ID = NetworkID
}
