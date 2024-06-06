//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-03.
//

import Foundation
import Sargon
import ComposableArchitecture

@Reducer
public struct ManageSecurityShieldsFeature {
	
	@Reducer(state: .equatable)
	public enum Destination {
		case newSecurityShield(NewSecurityShieldFeature)
	}
	
	@ObservableState
	public struct State {
		@SharedReader(.shields) var shields
		@Presents var destination: Destination.State?
	}
	
	public enum Action: ViewAction {
		public enum InternalAction {
			case newShield(preset: Shield?)
		}
		public enum ViewAction {
			case addNewButtonTapped
			case addSampleShieldButtonTapped
			case addSampleOtherShieldButtonTapped
		}
		case destination(PresentationAction<Destination.Action>)
		case view(ViewAction)
		case `internal`(InternalAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addSampleShieldButtonTapped):
				return .send(.internal(.newShield(preset: Shield.sample)))

			case .view(.addSampleOtherShieldButtonTapped):
				return .send(.internal(.newShield(preset: Shield.sampleOther)))

			case .view(.addNewButtonTapped):
				return .send(.internal(.newShield(preset: nil)))
				
			case let .internal(.newShield(preset)):
				state.destination = .newSecurityShield(NewSecurityShieldFeature.State(preset: preset))
				return .none
				
			case .destination:
				return .none
			}
		}
	}
}


extension ManageSecurityShieldsFeature {
	
	public typealias HostingFeature = ManageSecurityShieldsFeature
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("Shields").font(.largeTitle)
				
				if store.shields.isEmpty {
					Text("You have no shields")
				} else {
					ScrollView {
						ForEach(store.shields, id: \.id) { shield in
							VStack {
								ShieldCardView(shield: shield)
							}
						}
					}
				}
				
				Spacer()
				
				Button("Add New") {
					send(.addNewButtonTapped)
				}
				Button("Add New Sample") {
					send(.addSampleShieldButtonTapped)
				}
				Button("Add New Sample Other") {
					send(.addSampleOtherShieldButtonTapped)
				}
			}
			.padding(.bottom, 100)
			.sheet(
				item: $store.scope(
					state: \.destination?.newSecurityShield,
					action: \.destination.newSecurityShield
				)
			) { store in
				NewSecurityShieldFeature.View(store: store)
			}
		}
	}
}

public struct ShieldCardView: SwiftUI.View {
	public let shield: Shield
	public var body: some SwiftUI.View {
		Text("Shield: \(shield.id)")
	}
}
