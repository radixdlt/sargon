import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

public protocol BaseEditFactorStateProtocol {
	var baseCurrentFactors: IdentifiedArrayOf<AnyDisplayableFactorSource> { get }
}

public protocol EditFactorStateProtocol: ObservableState & Equatable where Key == PersistenceKeyDefault<SargonKey<IdentifiedArrayOf<F>>> {
	associatedtype Key
	associatedtype F: DisplayableFactorSource
	static var key: Key { get }
	var currentFactors: IdentifiedArrayOf<F> { get }
}
extension EditFactorStateProtocol {
	public static var factorKind: FactorSourceKind {
		F.kind
	}
}
extension AnyDisplayableFactorSource {
	init(factor: some DisplayableFactorSource) {
		self.init(hint: factor.hint, factorSource: factor.asGeneral)
	}
}

@ObservableState
public struct AnyEditFactorState: BaseEditFactorStateProtocol {
	public let factorKind: FactorSourceKind
	private let getFactors: () -> IdentifiedArrayOf<AnyDisplayableFactorSource>
	public var baseCurrentFactors: IdentifiedArrayOf<AnyDisplayableFactorSource> { getFactors() }
	
	private init<S>(_ state: S) where S: EditFactorStateProtocol {
		self.getFactors = {
			state.currentFactors.map(AnyDisplayableFactorSource.init).asIdentified()
		}
		self.factorKind = S.factorKind
	}
	
	public init(kind: FactorSourceKind) {
		switch kind {
		case .device:
			self.init(DeviceFS())
		case .ledgerHqHardwareWallet:
			self.init(LedgerFS())
		default: fatalError("Unsupported kind")
		}
	}
}

@ObservableState
public struct DeviceFS: EditFactorStateProtocol {
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
public struct LedgerFS: EditFactorStateProtocol {
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


public struct SpecificFactorSourcesFeature: Reducer & Equatable {
	public typealias State = AnyEditFactorState
	
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
				return .send(.delegate(.addNew(state.factorKind)))
		
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
		
		var factors: IdentifiedArrayOf<AnyDisplayableFactorSource> {
			store.state.baseCurrentFactors
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("\(store.factorKind) Factors").font(.largeTitle)
		
				if factors.isEmpty {
					Text("You have no factors")
				} else {
					ScrollView {
						ForEach(factors, id: \.id) { factorSource in
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

