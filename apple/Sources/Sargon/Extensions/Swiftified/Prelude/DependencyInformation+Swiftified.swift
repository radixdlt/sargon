import SargonUniFFI

// MARK: - DependencyInformation + SargonModel
extension DependencyInformation: SargonModel {}

// MARK: - DependencyInformation + CustomStringConvertible
extension DependencyInformation: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
