import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

public protocol Foo: ObservableState & Equatable where Key == PersistenceKeyDefault<SargonKey<IdentifiedArrayOf<F>>> {
	associatedtype Key
	associatedtype F: DisplayableFactorSource
	static var key: Key { get }
	var currentFactors: IdentifiedArrayOf<F> { get }
}
extension Foo {
	public static var factorKind: FactorSourceKind {
		F.kind
	}
}

@ObservableState
public struct DeviceFS: Foo {
	public typealias F = DeviceFactorSource
	public static let key: Key = .sharedDeviceFactorSources
	@SharedReader(key) public var factors
	public var currentFactors: IdentifiedArrayOf<F> {
		factors
	}
}
extension DeviceFactorSourceHint: FactorSourceHint {
	public func display() -> any SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Device Name", name)
			Labeled("Device Model", model)
			Labeled("#Mnemonic Words", mnemonicWordCount.rawValue)
			if let systemVersion {
				Labeled("iOS", systemVersion)
			}
			if let hostAppVersion {
				Labeled("App Version", hostAppVersion)
			}
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension DeviceFactorSource: DisplayableFactorSource {}


@ObservableState
public struct LedgerFS: Foo {
	public typealias F = LedgerHardwareWalletFactorSource
	public static let key: Key = .sharedLedgerFactorSources
	@SharedReader(key) public var factors
	public var currentFactors: IdentifiedArrayOf<F> {
		factors
	}
}
extension LedgerHardwareWalletHint: FactorSourceHint {
	public func display() -> any SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Ledger Name", name)
			Labeled("Ledger Model", model)
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension LedgerHardwareWalletFactorSource: DisplayableFactorSource {}


public typealias DeviceFactorSourcesFeature = SpecificFactorSourcesFeature<DeviceFS>
public typealias LedgerFactorSourcesFeature = SpecificFactorSourcesFeature<LedgerFS>

public struct SpecificFactorSourcesFeature<_State: Foo>: Reducer {
	public typealias State = _State
	
	public static var factorKind: FactorSourceKind { State.factorKind }
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case addNewButtonTapped
		}
		
		case view(ViewAction)
		
		@CasePathable
		public enum DelegateAction {
			case addNew(FactorSourceKind)
		}
		
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addNewButtonTapped):
				return .send(.delegate(.addNew(Self.factorKind)))
		
			default:
				return .none
				
			}
		}
	}
}

extension SpecificFactorSourcesFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<HostingFeature>
		
		typealias F = HostingFeature.State.F
		var factors: IdentifiedArrayOf<F> {
			store.state.currentFactors
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("\(HostingFeature.factorKind) Factors").font(.largeTitle)
		
				if factors.isEmpty {
					Text("You have no factors")
				} else {
					ScrollView {
						ForEach(factors) { factorSource in
							VStack {
								FactorSourceCardView(factorSource: factorSource)
							}
						}
					}
				}
				
				Spacer()
		   
				Button("Add New") {
					send(.addNewButtonTapped)
				}
			}
			.padding(.bottom, 100)
		}
	}
	
}

