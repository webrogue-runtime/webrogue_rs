@main
public class WebrogueAppDelegate: SDLUIKitDelegate {
    static var shared: WebrogueAppDelegate?
    var webrogueWindow: UIWindow?

    override public func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey : Any]? = nil
    ) -> Bool {
        WebrogueAppDelegate.shared = self

        let result = super.application(
            application,
            didFinishLaunchingWithOptions: launchOptions
        )
        webrogueWindow = UIWindow(frame: UIScreen.main.bounds)
        window = webrogueWindow
        webrogueWindow?.rootViewController = WebrogueUIViewController()
        webrogueWindow?.makeKeyAndVisible()
        return result
    }

    func runGame(completion: ((Int) -> Void)? = nil) {
        DispatchQueue.main.asyncAfter(deadline: .now()) {
            let ret_code = Int(webrogueMain())
            completion?(ret_code)
        }
    }
}
