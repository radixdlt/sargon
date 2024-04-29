import ComposableArchitecture
import Planbok
import SwiftUI

@main
struct PlanbokApp: App {
	var body: some Scene {
		WindowGroup {
			AppFeature.View(
				store: Store(
					initialState: AppFeature.State()
				) {
					AppFeature()
				}
			)
			.textFieldStyle(.roundedBorder)
			.buttonStyle(.borderedProminent)
		}
	}
}
