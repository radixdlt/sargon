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
	public struct State: Equatable {
		@Presents var destination: Destination.State?
		public var bip39Passphrase = ""
		public var phrase = ""
		public var mnemonic: Mnemonic? {
			try? Mnemonic(phrase: phrase)
		}
		public var mnemonicWithPassphrase: MnemonicWithPassphrase? {
			guard let mnemonic else { return nil }
			return MnemonicWithPassphrase(mnemonic: mnemonic, passphrase: bip39Passphrase)
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
			case confirmed(MnemonicWithPassphrase)
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
						ButtonState<Destination.PrefillMnemonicAlert>(action: action, label: {
							TextState("Prefill \(action.rawValue)")
						})
					}
				))
				return .none
			
			case .view(.confirmMnemonicButtonTapped):
				guard let mnemonicWithPassphrase = state.mnemonicWithPassphrase else { return .none }
				return .send(.delegate(.confirmed(mnemonicWithPassphrase)))
				
			case .delegate:
				return .none
				
			case let .destination(.presented(.prefillMnemonic(prefillAction))):
				let mnemonic = switch prefillAction {
				case .device24: Mnemonic.sampleDevice
				case .device24Other: Mnemonic.sampleDeviceOther
				case .arculus: Mnemonic.sampleArculus
				case .arculusOther: Mnemonic.sampleArculusOther
				case .device12: Mnemonic.sampleDevice12Words
				case .device12Other: Mnemonic.sampleDevice12WordsOther
				case .ledger: Mnemonic.sampleLedger
				case .ledgerOther: Mnemonic.sampleLedgerOther
				case .securityQuestions: Mnemonic.sampleSecurityQuestions
				case .securityQuestionsOther: Mnemonic.sampleSecurityQuestionsOther
				case .offDevice: Mnemonic.sampleOffDeviceMnemonic
				case .offDeviceOther: Mnemonic.sampleOffDeviceMnemonicOther
				}
				return .send(.view(.phraseChanged(mnemonic.phrase)))
				
			case .destination(.dismiss):
				state.destination = nil
				return .none
			case .destination:
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
			.padding()
			.buttonStyle(.borderedProminent)
			.alert($store.scope(state: \.destination?.prefillMnemonic, action: \.destination.prefillMnemonic))
		}
	}
}
