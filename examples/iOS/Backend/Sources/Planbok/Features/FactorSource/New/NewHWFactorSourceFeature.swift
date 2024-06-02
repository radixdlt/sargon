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
public struct NewHWFactorSourceFeature {
	@ObservableState
	public struct State {
		public let kind: FactorSourceKind
		public var inputMnemonic: InputMnemonicFeature.State
	}
	
	public enum Action {
		case inputMnemonic(InputMnemonicFeature.Action)
	}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.inputMnemonic, action: \.inputMnemonic) {
			InputMnemonicFeature()
		}
		
		Reduce {
			state,
			action in
			switch action {
			case let .inputMnemonic(.delegate(.confirmed(mnemonicWithPassphrase))):
				let factorSource: FactorSource = switch state.kind {
				case .device:
					let deviceInfo = DeviceInfo.sample
					let deviceFS: DeviceFactorSource = mnemonicWithPassphrase.mnemonic.wordCount == .twentyFour ? DeviceFactorSource.babylon(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						isMain: false,
						deviceInfo: deviceInfo
					) : DeviceFactorSource.olympia(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						deviceInfo: deviceInfo
					)
					FactorSource.device(value: deviceFS)
				case .ledger:
					LedgerHardwareWalletFactorSource.
				}
			case .inputMnemonic(_):
				return .none
			}
		}
	}
	
}

extension NewHWFactorSourceFeature {
	public typealias HostingFeature = Self
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("New \(store.state.kind) Factor")
				InputMnemonicFeature.View(store: store.scope(state: \.inputMnemonic, action: \.inputMnemonic))
			}
		}
	}
}
