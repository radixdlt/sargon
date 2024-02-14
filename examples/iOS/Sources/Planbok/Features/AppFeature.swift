//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-02-14.
//

import Foundation
import SwiftUI
import ComposableArchitecture
import Sargon
import SargonUniFFI

@Reducer
public struct AppFeature {
	public init() {}
	
	@ObservableState
	public struct State {
		public var decimal: Sargon.Decimal
		public init() {
			decimal = newDecimalFromString(string: "3.1416")
		}
	}
	
	public enum Action {}
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<AppFeature>
		public init(store: StoreOf<AppFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			Text("Decimal: \(store.decimal)")
		}
	}
}
