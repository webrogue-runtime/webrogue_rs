@objc 
class AngleHelperSystem: NSObject {
    @objc
    public override init() {
        SDL_Init(SDL_INIT_VIDEO);
    }

    @objc
    public func makeWindow() -> AngleHelperWindow {
        AngleHelperWindow()
    }
}
