//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-28.
//

import Foundation
import SargonUniFFI

extension ProfileNetwork {
	public func detailsForAuthorizedDapp(
		_ dapp: AuthorizedDapp
	) throws -> AuthorizedDappDetailed {
		try profileNetworkDetailsForAuthorizedDapp(
			profileNetwork: self,
			dapp: dapp
		)
	}
}
