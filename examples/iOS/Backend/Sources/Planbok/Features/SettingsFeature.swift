//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import ComposableArchitecture

#if DEBUG
@Reducer
public struct SettingsFeature {
	
	@ObservableState
	public struct State: Equatable {}
	
	public enum Action {}
	
	public init() {}
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<SettingsFeature>
		public var body: some SwiftUI.View {
			VStack {
				Text("TODO SETTINGS")
			}
		}
	}
}

#endif
