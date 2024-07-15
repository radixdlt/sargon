import SargonUniFFI

extension ResourceSpecifier {
    public var resourceAddress: ResourceAddress {
        resourceSpecifierGetAddress(specifier: self)
    }
}
