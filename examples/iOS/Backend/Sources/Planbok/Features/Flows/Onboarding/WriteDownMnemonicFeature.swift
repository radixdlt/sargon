//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

public typealias Result<T> = Swift.Result<T, CommonError>

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
		public enum InternalAction {
			case loadedPrivateHDFactor(PrivateHierarchicalDeterministicFactorSource)
		}
		case delegate(DelegateAction)
		case view(ViewAction)
		case `internal`(InternalAction)
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
	
	@Dependency(MnemonicClient.self) var mnemonicClient
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .internal(.loadedPrivateHDFactor(privateHDFactor)):
				state.mnemonic = privateHDFactor.mnemonicWithPassphrase.mnemonic.phrase
				return .none
				
			case .view(.revealMnemonicButtonTapped):
				return .run { send in
					let id = try SargonOS.shared.profile().factorSources.first!.id.extract(as: FactorSourceIdFromHash.self)
					let privateHDFactor = try await mnemonicClient.loadMnemonic(id)
					await send(.internal(.loadedPrivateHDFactor(privateHDFactor)))
				} catch: { error, send in
					fatalError("error \(error)")
				}
			case .view(.continueButtonTapped):
				return .send(.delegate(.done))
			case .delegate:
				return .none
			}
		}
	}
}

import Sargon
import DependenciesMacros

@DependencyClient
public struct MnemonicClient: Sendable {
	public typealias LoadMnemonic = @Sendable (FactorSourceIDFromHash) async throws -> PrivateHierarchicalDeterministicFactorSource
	public var loadMnemonic: LoadMnemonic
}
extension MnemonicClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			loadMnemonic: { id in
				try await os.loadPrivateDeviceFactorSourceById(id: id)
			}
		)
	}
}

@DependencyClient
public struct AccountsClient: Sendable {
	public typealias GetAccounts = @Sendable () -> Accounts
	public typealias AccountsStream = @Sendable () -> AsyncStream<Accounts>
	public typealias CreateAndSaveAccount = @Sendable (NetworkID, DisplayName) async throws -> Account

	public var getAccounts: GetAccounts
	public var accountsStream: AccountsStream
	public var createAndSaveAccount: CreateAndSaveAccount
}
extension AccountsClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		let getAccounts: GetAccounts = {
			os.accounts()
		}
		return Self(
			getAccounts: getAccounts,
			accountsStream: {
				AsyncStream<Accounts> { continuation in
					Task {
						for await _ in await EventBus.shared.notifications() {
							continuation.yield(getAccounts())
						}
					}
				}
			},
			createAndSaveAccount: {
				try await os.createAndSaveNewAccount(networkId: $0, name: $1)
			}
		)
	}
}
