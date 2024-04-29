import Foundation
import SargonUniFFI

// MARK: - DepositorsAllowList + SargonModel
extension DepositorsAllowList: SargonModel {}

// MARK: - DepositorsAllowList + CanBeEmptyIdentifiedCollection
extension DepositorsAllowList: CanBeEmptyIdentifiedCollection {
	public typealias Element = ResourceOrNonFungible
}
