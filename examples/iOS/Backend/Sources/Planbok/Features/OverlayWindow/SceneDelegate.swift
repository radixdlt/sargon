import ComposableArchitecture
import SwiftUI

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
            // avoids unimplemented("OverlayWindowClient.isUserInteractionEnabled")
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
            ))

        let overlayWindow = UIWindow(windowScene: scene)
        overlayWindow.rootViewController = UIHostingController(rootView: overlayView)
        overlayWindow.rootViewController?.view.backgroundColor = .clear
        overlayWindow.windowLevel = .normal + 1
        overlayWindow.isUserInteractionEnabled = false
        overlayWindow.makeKeyAndVisible()

        self.overlayWindow = overlayWindow
    }
}
