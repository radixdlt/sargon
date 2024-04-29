import Foundation
import SargonUniFFI

extension AuthorizedDapps {
	public init(_ elements: [AuthorizedDapp]) {
		self = newAuthorizedDapps(authorizedDapps: elements)
	}

	public init(element: AuthorizedDapp) {
		self = newAuthorizedDappsWithAuthorizedDapp(authorizedDapp: element)
	}

	public var elements: [AuthorizedDapp] {
		authorizedDappsGetElements(authorizedDapps: self)
	}

	public func appending(_ authorizedDapp: AuthorizedDapp) -> Self {
		newAuthorizedDappsByAppending(authorizedDapp: authorizedDapp, to: self)
	}

	public func updatingOrInserting(element authorizedDapp: Element, at index: Int) -> Self {
		newAuthorizedDappsByUpdatingOrInsertingAtIndex(authorizedDapp: authorizedDapp, to: self, index: UInt64(index))
	}

	public func updatingOrAppending(_ authorizedDapp: AuthorizedDapp) -> Self {
		newAuthorizedDappsByUpdatingOrAppending(authorizedDapp: authorizedDapp, to: self)
	}

	public func removing(_ id: AuthorizedDapp.ID) -> Self {
		newAuthorizedDappsRemovedById(idOfAuthorizedDapp: id, from: self)
	}

	public func removing(element dapp: AuthorizedDapp) -> Self {
		newAuthorizedDappsRemovedElement(authorizedDapp: dapp, from: self)
	}

	public func get(id: AuthorizedDapp.ID) -> AuthorizedDapp? {
		authorizedDappsGetAuthorizedDappById(authorizedDapps: self, id: id)
	}

	public var count: Int {
		Int(authorizedDappsElementCount(authorizedDapps: self))
	}
}
