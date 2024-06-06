//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-06.
//

import Foundation
import Sargon
import ComposableArchitecture

@Reducer
public struct NewSecurityShieldFeature {
	
	@ObservableState
	public struct State: Equatable {
		public init(preset: Shield?) {}
	}
	
	public enum Action {}
}

extension NewSecurityShieldFeature {
	public typealias HostingFeature = Self
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			Text("NewSecurityShieldFeature")
		}
	}
}
