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
		case newSecurityShield(NewSecurityShieldCoordinator)
	}
	
	@ObservableState
	public struct State {
		@SharedReader(.shields) var shields
		@SharedReader(.factorSources) var factorSources
		@Presents var destination: Destination.State?
		
		public init(copyAndEdit preset: Shield? = nil) {
			if let preset {
				destination = .newSecurityShield(NewSecurityShieldCoordinator.State(copyAndEdit: preset))
			}
		}
		
		public var canAddSampleShields: Bool {
			// FIXME: cleanup
			var used: [FactorSource] = []
			let m = Shield.sample.matrixOfFactors
			used.append(contentsOf: m.primaryRole.thresholdFactors)
			used.append(contentsOf: m.primaryRole.overrideFactors)
			used.append(contentsOf: m.recoveryRole.thresholdFactors)
			used.append(contentsOf: m.recoveryRole.overrideFactors)
			used.append(contentsOf: m.confirmationRole.thresholdFactors)
			used.append(contentsOf: m.confirmationRole.overrideFactors)
			let usedIDs = Set(used.map(\.id))
			return Set(factorSources.map(\.id)).isSuperset(of: usedIDs)
		}
	}
	
	public enum Action: ViewAction {
		public enum InternalAction {
			case newShield(preset: Shield?)
		}
		public enum ViewAction {
			case shieldTapped(Shield)
			case addNewButtonTapped
			case addSampleShieldButtonTapped
			case addSampleOtherShieldButtonTapped
		}
		case destination(PresentationAction<Destination.Action>)
		public enum DelegateAction {
			public enum Navigate {
				case toDetailsForShield(Shield)
			}
			case navigate(Navigate)
		}
		case delegate(DelegateAction)
		case view(ViewAction)
		case `internal`(InternalAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.shieldTapped(shield)):
				return .send(.delegate(.navigate(.toDetailsForShield(shield))))
				
			case .view(.addSampleShieldButtonTapped):
				return .send(.internal(.newShield(preset: Shield.sample)))

			case .view(.addSampleOtherShieldButtonTapped):
				return .send(.internal(.newShield(preset: Shield.sampleOther)))

			case .view(.addNewButtonTapped):
				return .send(.internal(.newShield(preset: nil)))
				
			case let .internal(.newShield(preset)):
				state.destination = .newSecurityShield(NewSecurityShieldCoordinator.State(copyAndEdit: preset))
				return .none
				
			case .destination(.presented(.newSecurityShield(.delegate(.done)))):
				state.destination = nil
				return .none
				
			case .delegate:
				return .none
			case .destination:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
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
				
				ScrollView {
					
					Text("Security shields are a combination of factors you can use to protect your accounts and personas.")
					
					Text("Here are your current security shields.")
					
					if store.shields.isEmpty {
						Text("You have no shields")
					} else {
						ForEach(store.shields, id: \.id) { shield in
							VStack {
								ShieldCardView(shield: shield) {
									send(.shieldTapped(shield))
								}
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
				.disabled(!store.canAddSampleShields)
				Button("Add New Sample Other") {
					send(.addSampleOtherShieldButtonTapped)
				}
				.disabled(!store.canAddSampleShields)
				if !store.canAddSampleShields {
					Text("Add ALL Sample Factors from Manage Factor Sources to be able to add sample shields").font(.footnote)
				}
			}
			.padding(.horizontal)
			.padding(.bottom, 100)
			.sheet(
				item: $store.scope(
					state: \.destination?.newSecurityShield,
					action: \.destination.newSecurityShield
				)
			) { store in
				NewSecurityShieldCoordinator.View(store: store)
				
			}
		}
	}
}

public struct ShieldCardView: SwiftUI.View {
	public let shield: Shield
	public let action: () -> Void
	public var body: some SwiftUI.View {
		Button(action: action, label: {
			HStack {
				Image(systemName: "lock.shield")
					.resizable()
					.imageScale(.large)
					.aspectRatio(contentMode: .fit)
					.frame(idealHeight: 50)
				Text("\(shield.metadata.displayName)")
					.font(.title2)
				Spacer()
				Image(systemName: "chevron.right")
			}
			.foregroundStyle(Color.app.blue1)
		})
		.buttonStyle(.plain)
		.padding()
	}
}
