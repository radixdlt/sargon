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
