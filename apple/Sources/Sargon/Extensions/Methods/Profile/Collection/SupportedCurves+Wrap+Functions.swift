//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension SupportedCurves {
	
	public init(_ elements: [Element]) throws {
		self = try newSupportedCurves(supportedCurves: elements)
	}
	
	public init(element: Element) {
		self = newSupportedCurvesWithSLIP10Curve(sLIP10Curve: element)
	}
	
	public var elements: [Element] {
		supportedCurvesGetElements(supportedCurves: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newSupportedCurvesByAppending(sLIP10Curve: element, to: self)
	}
	
	public func updatingOrInserting(element sLIP10Curve: Element, at index: Int) -> Self {
		newSupportedCurvesByUpdatingOrInsertingAtIndex(sLIP10Curve: sLIP10Curve, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newSupportedCurvesByUpdatingOrAppending(sLIP10Curve: element, to: self)
	}
	
	public func removingElementByID(_ id: Element.ID) throws -> Self {
		try newSupportedCurvesRemovedById(idOfSLIP10Curve: id, from: self)
	}
	
	public func removing(element: Element) throws -> Self {
		try newSupportedCurvesRemovedElement(sLIP10Curve: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		supportedCurvesGetSLIP10CurveById(supportedCurves: self, id: id)
	}
	
	public var count: Int {
		Int(supportedCurvesElementCount(supportedCurves: self))
	}
}
