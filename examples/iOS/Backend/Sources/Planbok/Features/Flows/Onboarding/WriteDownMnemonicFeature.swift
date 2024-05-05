//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

@Reducer
public struct WriteDownMnemonicFeature {
	
	@Dependency(\.keychain) var keychain

	public init() {}
	
	@ObservableState
	public struct State: Equatable {
		public var mnemonic: String?
		public init() {
		}
	}
	
	public enum Action: ViewAction {
		public enum DelegateAction {
			case done
		}
		public enum ViewAction {
			case revealMnemonicButtonTapped
			case continueButtonTapped
		}
		case delegate(DelegateAction)
		case view(ViewAction)
	}
	
	@ViewAction(for: WriteDownMnemonicFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<WriteDownMnemonicFeature>
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Write down your mnemonic on a piece of paper and put it in a safe")
					.font(.title)
				Spacer()
				if let mnemonic = store.state.mnemonic {
					Text("`\(mnemonic)`")
						.border(.yellow)
				} else {
					Button("Reveal") {
						send(.revealMnemonicButtonTapped)
					}
				}
				Spacer()
				Button("Continue") {
					send(.continueButtonTapped)
				}
			}
			.padding()
		}
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.revealMnemonicButtonTapped):
//				let wallet = state.walletHolder.wallet
//			
//				do {
//					let bdfsMnemonic = try wallet.mainBdfsMnemonicWithPassphrase()
//					state.mnemonic = bdfsMnemonic.mnemonic.phrase
//				} catch {
//					fatalError("handle error: \(error)")
//				}
				return .none
			case .view(.continueButtonTapped):
				return .send(.delegate(.done))
			case .delegate:
				return .none
			}
		}
	}
}
