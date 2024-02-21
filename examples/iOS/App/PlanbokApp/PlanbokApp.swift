//
//  PlanbokApp.swift
//  Planbok
//
//  Created by Alexander Cyon on 2024-02-14.
//

import SwiftUI
import Planbok
import ComposableArchitecture

@main
struct PlanbokApp: App {
    var body: some Scene {
        WindowGroup {
			VStack {
				AppFeature.View(
					store: Store(
						initialState: AppFeature.State()
					) {
						AppFeature()
					}
				)
			}
			.textFieldStyle(.roundedBorder)
			.buttonStyle(.borderedProminent)
        }
    }
}
