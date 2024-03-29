import SargonUniFFI

extension DependencyInformation: SargonModel {}
extension DependencyInformation: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
