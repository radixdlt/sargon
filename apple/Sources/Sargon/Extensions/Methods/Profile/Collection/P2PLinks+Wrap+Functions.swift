//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension P2pLinks {
	public init(_ elements: [P2pLink]) {
		self = newP2PLinks(p2PLinks: elements)
	}
	
	public init(element: P2pLink) {
		self = newP2PLinksWithP2PLink(p2PLink: element)
	}
	
	public func allElements() -> [P2pLink] {
		p2PLinksGetElements(p2PLinks: self)
	}
	
	public func appending(_ link: P2pLink) -> Self {
		newP2PLinksByAppending(p2PLink: link, to: self)
	}
	
	public func updatingOrInserting(element p2PLink: Element, at index: Int) -> Self {
		newP2PLinksByUpdatingOrInsertingAtIndex(p2PLink: p2PLink, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ link: P2pLink) -> Self {
		newP2PLinksByUpdatingOrAppending(p2PLink: link, to: self)
	}
	
	public func removing(_ id: P2PLink.ID) -> Self {
		newP2PLinksRemovedById(idOfP2PLink: id, from: self)
	}
	
	public func removing(element link: P2pLink) -> Self {
		newP2PLinksRemovedElement(p2PLink: link, from: self)
	}
	
	public func get(id: P2pLink.ID) -> P2pLink? {
		p2PLinksGetP2PLinkById(p2PLinks: self, id: id)
	}
	
	public var count: Int {
		Int(p2PLinksElementCount(p2PLinks: self))
	}
}
