//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension AssetsExceptionList {
	public init(_ elements: [Element]) {
		self = newAssetsExceptionList(assetsExceptionList: elements)
	}
	
	public init(element: Element) {
		self = newAssetsExceptionListWithAssetException(assetException: element)
	}
	
	public func allElements() -> [Element] {
		assetsExceptionListGetElements(assetsExceptionList: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newAssetsExceptionListByAppending(assetException: element, to: self)
	}
	
	public func updatingOrInserting(element: Element, at index: Int) -> Self {
		newAssetsExceptionListByUpdatingOrInsertingAtIndex(
			assetException: element,
			to: self,
			index: UInt64(
				index
			)
		)
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newAssetsExceptionListByUpdatingOrAppending(assetException: element, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newAssetsExceptionListRemovedById(idOfAssetException: id, from: self)
	}
	
	public func removing(element: Element) -> Self {
		newAssetsExceptionListRemovedElement(assetException: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		assetsExceptionListGetAssetExceptionById(assetsExceptionList: self, id: id)
	}
	
	public var count: Int {
		Int(assetsExceptionListElementCount(assetsExceptionList: self))
	}
}
