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
		
	}
	
	public enum Action {}
}


extension ManageSecurityShieldsFeature {
	
	public typealias HostingFeature = ManageSecurityShieldsFeature
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
	}
}
