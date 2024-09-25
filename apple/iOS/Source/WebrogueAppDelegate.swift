@main
public class WebrogueAppDelegate: SDLUIKitDelegate {
    static var shared: WebrogueAppDelegate!
    var webrogueWindow: UIWindow!
    var isWebrogueWindowVisible = true

    override public var window: UIWindow! {
        get {
            isWebrogueWindowVisible ? webrogueWindow : super.window
        }
        set {}
    }

    override public func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil
    ) -> Bool {
        WebrogueAppDelegate.shared = self

        let result = super.application(
            application,
            didFinishLaunchingWithOptions: launchOptions
        )
        webrogueWindow = UIWindow(frame: UIScreen.main.bounds)
        webrogueWindow.rootViewController = WebrogueUIViewController()
        webrogueWindow.makeKeyAndVisible()
        return result
    }

    func runGame(path: String, completion: ((Int) -> Void)? = nil) {
        DispatchQueue.main.asyncAfter(deadline: .now()) {
            self.isWebrogueWindowVisible = false
            let ret_code = path.utf8CString.withUnsafeBufferPointer {
                Int(webrogueMain($0.baseAddress!))
            }
            self.isWebrogueWindowVisible = true
            completion?(ret_code)
        }
    }
}
