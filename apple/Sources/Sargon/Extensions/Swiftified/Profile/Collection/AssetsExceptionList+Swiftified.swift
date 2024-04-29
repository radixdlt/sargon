import Foundation
import SargonUniFFI

// MARK: - AssetsExceptionList + SargonModel
extension AssetsExceptionList: SargonModel {}

// MARK: - AssetsExceptionList + CanBeEmptyIdentifiedCollection
extension AssetsExceptionList: CanBeEmptyIdentifiedCollection {
	public typealias Element = AssetException
}
