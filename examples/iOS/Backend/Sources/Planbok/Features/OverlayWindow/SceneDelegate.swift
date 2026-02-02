import ComposableArchitecture
import SwiftUI

// MARK: - SceneDelegate
public final class SceneDelegate: NSObject, UIWindowSceneDelegate, ObservableObject {
	public weak var windowScene: UIWindowScene?
	public var overlayWindow: UIWindow?

	public func scene(
		_ scene: UIScene,
		willConnectTo session: UISceneSession,
		options connectionOptions: UIScene.ConnectionOptions
	) {
		windowScene = scene as? UIWindowScene
		if
			let windowScene,
			!_XCTIsTesting
		{
			overlayWindow(in: windowScene)
		}
	}

	func overlayWindow(in scene: UIWindowScene) {
		let overlayView = OverlayFeature.View(
			store: .init(
				initialState: .init(),
				reducer: OverlayFeature.init
			)
		)

		let overlayWindow = UIWindow(windowScene: scene)
		overlayWindow.rootViewController = UIHostingController(rootView: overlayView)
		overlayWindow.rootViewController?.view.backgroundColor = .clear
		overlayWindow.windowLevel = .normal + 1
		overlayWindow.isUserInteractionEnabled = false
		overlayWindow.backgroundColor = .clear
		overlayWindow.makeKeyAndVisible()
		self.overlayWindow = overlayWindow
	}
}

// MARK: - TransparentBackground
struct TransparentBackground: UIViewRepresentable {
	func makeUIView(context: Context) -> UIView {
		let view = UIView()
		DispatchQueue.main.async {
			view.backgroundColor = .clear
			view.superview?.backgroundColor = .clear
			view.superview?.superview?.backgroundColor = .clear
			view.superview?.superview?.superview?.backgroundColor = .clear
			view.superview?.superview?.superview?.superview?.backgroundColor = .clear
		}
		return view
	}

	func updateUIView(_ uiView: UIView, context: Context) {}
}
