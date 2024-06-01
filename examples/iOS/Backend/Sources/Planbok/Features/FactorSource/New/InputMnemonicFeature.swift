//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-01.
//

import Foundation
import ComposableArchitecture
import Sargon

@Reducer
public struct InputMnemonicFeature {
	
	
	@Reducer(state: .equatable)
	public enum Destination {
		case prefillMnemonic(AlertState<PrefillMnemonicAlert>)
		
		public enum PrefillMnemonicAlert: String, CaseIterable {
			case device24
			case device24Other
			case device12
			case device12Other
			case offDevice
			case offDeviceOther
			case securityQuestions
			case securityQuestionsOther
			case ledger
			case ledgerOther
			case arculus
			case arculusOther
		}
	}
	
	@ObservableState
	public struct State {
		@Presents var destination: Destination.State?
		public var phrase = ""
		public var mnemonic: Mnemonic? {
			try? Mnemonic(phrase: phrase)
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case phraseChanged(String)
			case prefillButtonTapped
			case confirmMnemonicButtonTapped
		}
		public enum DelegateAction {
			case confirmed(Mnemonic)
		}
		case view(ViewAction)
		case delegate(DelegateAction)
		case destination(PresentationAction<Destination.Action>)
	   
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.phraseChanged(phrase)):
				state.phrase = phrase
				return .none
				
			case .view(.prefillButtonTapped):
				state.destination = .prefillMnemonic(.init(
					title: TextState("Prefill Mnemonic"),
					message: TextState("Will fill in the phrase"),
					buttons: [
						.cancel(TextState("Cancel"))
					] + Destination.PrefillMnemonicAlert.allCases.map { action in
						ButtonState<Destination.PrefillMnemonicAlert>.init(action: action, label: {
							TextState("Prefill \(action.rawValue)")
						})
					}
				))
				return .none
			
			case .view(.confirmMnemonicButtonTapped):
				guard let mnemonic = state.mnemonic else { return .none }
				return .send(.delegate(.confirmed(mnemonic)))
				
			case .delegate(_):
				return .none
				
			case let .destination(.presented(.prefillMnemonic(prefillAction))):
				let mnemonic = switch prefillAction {
				case .arculus: Mnemonic.sample
				default:  Mnemonic.sampleOther
				}
				return .send(.view(.phraseChanged(mnemonic.phrase)))
				
			case .destination(.dismiss):
				state.destination = nil
				return .none
			case .destination(_):
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
}


extension InputMnemonicFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Input Mnemonic").font(.largeTitle)
				
				LabeledTextField(label: "Phrase", text: $store.phrase.sending(\.view.phraseChanged))
				
				Button("Prefill") {
					send(.prefillButtonTapped)
				}
				
				Button("Confirm") {
					send(.confirmMnemonicButtonTapped)
				}
				.disabled(store.mnemonic == nil)
			}
			.alert($store.scope(state: \.destination?.prefillMnemonic, action: \.destination.prefillMnemonic))
		}
	}
}
