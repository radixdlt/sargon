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
	
	@ObservableState
	public struct State {
		@SharedReader(.shields) var shields
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case addSampleShieldButtonTapped
			case addSampleOtherShieldButtonTapped
		}
		case view(ViewAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addSampleShieldButtonTapped):
				return .none
			case .view(.addSampleOtherShieldButtonTapped):
				return .none
			}
		}
	}
}


extension ManageSecurityShieldsFeature {
	
	public typealias HostingFeature = ManageSecurityShieldsFeature
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				ForEach(store.shields) { shield in
					Text("Shield id: \(shield.id)")
				}
				Button("Add Sample Shield") {
					send(.addSampleShieldButtonTapped)
				}
				Button("Add Sample Other Shield") {
					send(.addSampleShieldButtonTapped)
				}
			}
		}
	}
}
