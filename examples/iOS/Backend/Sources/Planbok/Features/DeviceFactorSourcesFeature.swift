import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct DeviceFactorSourcesFeature {

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.deviceFactorSources) var deviceFactorSources
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case deviceFactorSourcesButtonTapped
		}
		
		case view(ViewAction)
		
		@CasePathable
		public enum DelegateAction {
			case navigate(Navigate)
			
			@CasePathable
			public enum Navigate {
				case toDeviceFactorSources
			}
		}
		
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.deviceFactorSourcesButtonTapped):
				return .send(.delegate(.navigate(.toDeviceFactorSources)))
				
		
			default:
				return .none
				
			}
		}
	}
}

extension DeviceFactorSourcesFeature {
	
	@ViewAction(for: DeviceFactorSourcesFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<DeviceFactorSourcesFeature>
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Device FactorSources").font(.largeTitle)
		
				if store.state.deviceFactorSources.isEmpty {
					Text("You have no DeviceFactorSources")
				} else {
					ScrollView {
						ForEach(store.state.deviceFactorSources) { factorSource in
							VStack {
								FactorSourceCardView(factorSource: factorSource)
							}
						}
					}
				}
		   
				
			}
			.padding(.bottom, 100)
		}
	}
	
}

public struct FactorSourceCardView<F: DisplayableFactorSource>: SwiftUI.View {
	public let factorSource: F
	public var body: some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Kind", factorSource.kind)
			Labeled("Added", factorSource.addedOn.formatted(.dateTime))
			Labeled("Last Used", factorSource.lastUsedOn.formatted(.dateTime))
			
			AnyView(factorSource.hint.display())
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
		.background(Color.orange)
		.padding()
	}
}

public protocol FactorSourceHint {
	func display() -> any SwiftUI.View
}
public protocol DisplayableFactorSource: FactorSourceProtocol {
	associatedtype Hint: FactorSourceHint
	var hint: Hint { get }
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

public struct Labeled: SwiftUI.View {
	public let title: String
	public let value: String
	public init<V>(_ title: String, _ value: V) where V: CustomStringConvertible {
		self.title = title
		self.value = value.description
	}
	public var body: some SwiftUI.View {
		HStack {
			Text("**\(title)**")
			Text("`\(value)`")
		}
		.multilineTextAlignment(.leading)
	}
}
