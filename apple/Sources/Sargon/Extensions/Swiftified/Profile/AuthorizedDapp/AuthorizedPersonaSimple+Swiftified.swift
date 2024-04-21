//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension AuthorizedPersonaSimple: SargonModel {}
extension AuthorizedPersonaSimple: Identifiable {
	public typealias ID = IdentityAddress
	public var id: ID {
		identityAddress
	}
}

extension AuthorizedPersonaSimple {
	public var networkID: NetworkID {
		identityAddress.networkID
	}
}

#if DEBUG
extension AuthorizedPersonaSimple {
	public static let sampleValuesMainnet: [Self] = [.sampleMainnet, .sampleMainnetOther]
	public static let sampleValuesStokenet: [Self] = [.sampleStokenet, .sampleStokenetOther]
	public static let sampleValues: [Self] = Self.sampleValuesMainnet + Self.sampleValuesStokenet
}
#endif // DEBUG
