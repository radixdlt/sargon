import SargonUniFFI

extension DependencyInformation {
	public func toString() -> String {
		dependencyInformationToString(info: self)
	}
}
