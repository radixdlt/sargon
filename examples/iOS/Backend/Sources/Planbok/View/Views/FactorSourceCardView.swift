//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-01.
//

import Foundation
import SwiftUI

public protocol FactorSourceHint {
	func display() -> any SwiftUI.View
}

public struct AnyDisplayableFactorSource: Hashable & Identifiable {
	public typealias ID = FactorSourceID
	public var id: ID {
		factorSource.id
	}
	public static func == (lhs: Self, rhs: Self) -> Bool {
		lhs.factorSource == rhs.factorSource
	}
	public func hash(into hasher: inout Hasher) {
		hasher.combine(factorSource)
	}
	public let hint: any FactorSourceHint
	public let factorSource: FactorSource
}

public protocol DisplayableFactorSource: FactorSourceProtocol & Identifiable where ID: Sendable {
	associatedtype Hint: FactorSourceHint
	var hint: Hint { get }
}

public struct FactorSourceCardView: SwiftUI.View {
	public let factorSource: AnyDisplayableFactorSource
	public var body: some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Kind", factorSource.factorSource.kind)
			Labeled("Added", factorSource.factorSource.addedOn.formatted(.dateTime))
			Labeled("Last Used", factorSource.factorSource.lastUsedOn.formatted(.dateTime))
			
			AnyView(factorSource.hint.display())
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
		.background(Color.orange)
		.padding()
	}
}

