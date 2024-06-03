//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-02.
//

import Foundation
import SargonUniFFI

extension TrustedContactFactorSource {
	public init(
		accountAddress: AccountAddress,
		contact: TrustedContactFactorSourceContact
	) {
		self = newTrustedContactFrom(
			accountAddress: accountAddress,
			contact: contact
		)
	}
}
