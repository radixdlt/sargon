import Foundation
import Sargon
import ComposableArchitecture

public typealias FactorSources = IdentifiedArrayOf<FactorSource>

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<FactorSources>> {
    public static var factorSources: Self {
        Self.sharedFactorSources
    }
}

extension PersistenceKeyDefault<SargonKey<FactorSources>> {
    public static let sharedFactorSources = Self(
        SargonKey(
            accessing: \.factorSources,
            fetchIf: \.affectsFactorSources
        ),
        FactorSources()
    )
}

extension SargonOS {
    
    public var factorSources: FactorSources {
        factorSources().asIdentified()
    }
    
}

extension EventKind {
    
    public var affectsFactorSources: Bool {
        eventKindAffectsFactorSources(eventKind: self)
    }
}
