//
//  PasswordFactorSource+Swiftified.swift
//
//
//  Created by Michael Bakogiannis on 7/10/24.
//

import Foundation
import SargonUniFFI

extension PasswordFactorSource: SargonModel {}
extension PasswordFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

extension PasswordFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .password

	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .password(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}

	public var asGeneral: FactorSource {
		.password(value: self)
	}

	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}

	public var factorSourceKind: FactorSourceKind {
		.password
	}

	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
