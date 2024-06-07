//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-06.
//

import Foundation
import Sargon
import ComposableArchitecture

public enum FactorThreshold: Hashable, Sendable {
	case any
	case all
	case threshold(UInt8)
}

public enum Factor: Hashable, Sendable, Identifiable {
	public enum ID: Hashable, Sendable {
		case placeholder(UUID)
		case factor(FactorSourceID)
	}
	case placeholder(UUID)
	case factor(FactorSource)
	var factorSource: FactorSource? {
		switch self {
		case .placeholder: return nil
		case let .factor(factor): return factor
		}
	}
	public var id: ID {
		switch self {
		case let .placeholder(id): .placeholder(id)
		case let .factor(factor): .factor(factor.id)
		}
	}
}

@Reducer
public struct PrimaryRoleFactorsFeature {
	@ObservableState
	public struct State: Equatable {
		
	
		
		@SharedReader(.factorSources) var allInProfile
		
		public typealias Factors = IdentifiedArrayOf<Factor>
		
		
		var allPicked: FactorSources {
			var picked = FactorSources()
			func addFrom(_ factors: Factors) {
				picked.append(contentsOf: factors.compactMap(\.factorSource))
			}
			addFrom(self.thresholdFactors)
			addFrom(self.overrideFactors)
			return picked
		}
		
		var available: FactorSources {
			let picked = Set(allPicked.map(\.id))
			return allInProfile.filter({ !picked.contains($0.id) }).asIdentified()
		}
		
		public var thresholdFactors: Factors = []
		public var threshold: FactorThreshold = .any
		public var overrideFactors: Factors = []
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case confirmButtonTapped
			case pickButtonTapped
		}
		public enum DelegateAction {
			case `continue`
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.confirmButtonTapped):
				return .send(.delegate(.continue))
			case .view(.pickButtonTapped):
				return .none
			case .delegate:
				return .none
			}
		}
	}
}

extension PrimaryRoleFactorsFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("Sign Transactions").font(.largeTitle)
				
				Text("These factors are required to withdraw your assets and log in to dApps.")
				
				FactorsBuilderView(factors: store.thresholdFactors, title: "Security", titleAction: { log.info("very important!") }, pickAction: {
					send(.pickButtonTapped)
				})
				
				Button("Confirm") {
					send(.confirmButtonTapped)
				}
				.buttonStyle(.borderedProminent)
			}
		}
	}
}

public struct FactorsBuilderView: SwiftUI.View {

	public var factors: IdentifiedArrayOf<Factor>

	public let title: LocalizedStringKey
	public let titleAction: () -> Void
	public let pickAction: () -> Void
	

	public var body: some SwiftUI.View {
		VStack {
			Button(
				action: titleAction,
				label: {
					Label(title, image: "info.circle")
				}
			)
			List {
				ForEach(factors) { factor in
					FactorView(
						factor: factor,
						pickAction: pickAction
					) {
						self.factors.remove(
							id: factor.id
						)
					}
				}
			}
			Button("Add factors") {
				self.factors.append(Factor.placeholder(.init()))
			}
		}
	}
	public struct FactorView: SwiftUI.View {
		public let factor: Factor
		public let pickAction: () -> Void
		public let removeAction: () -> Void
		public var body: some SwiftUI.View {
			HStack {
				switch factor {
					case .placeholder:
					Button("<PICK>", action: pickAction)
				case let .factor(factorSource):
					Text("\(factorSource.kind)")
				}
				Button("X", action: removeAction)
			}
			
		}
	}
}

