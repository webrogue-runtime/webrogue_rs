import SwiftUI

class WebrogueUIViewController: UIViewController {

    let swiftUIView = UIHostingController(rootView: WebrogueView())

    override func viewWillAppear(_ animated: Bool) {
        view.backgroundColor = UIColor(named: "customBackgroundColor")
        super.viewWillAppear(animated)
    }

    override func viewDidLoad() {
        let subview = swiftUIView.view!
        view.addSubview(subview)
        subview.translatesAutoresizingMaskIntoConstraints = false
        view.addConstraints([
            NSLayoutConstraint(
                item: subview,
                attribute: NSLayoutConstraint.Attribute.left,
                relatedBy: NSLayoutConstraint.Relation.equal,
                toItem: view,
                attribute: NSLayoutConstraint.Attribute.left,
                multiplier: 1,
                constant: 0
            ),
            NSLayoutConstraint(
                item: subview, 
                attribute: NSLayoutConstraint.Attribute.right,
                relatedBy: NSLayoutConstraint.Relation.equal,
                toItem: view,
                attribute: NSLayoutConstraint.Attribute.right,
                multiplier: 1,
                constant: 0
            ),
            NSLayoutConstraint(
                item: subview,
                attribute: NSLayoutConstraint.Attribute.top,
                relatedBy: NSLayoutConstraint.Relation.equal,
                toItem: view,
                attribute: NSLayoutConstraint.Attribute.top,
                multiplier: 1,
                constant: 0
            ),
            NSLayoutConstraint(
                item: subview,
                attribute: NSLayoutConstraint.Attribute.bottom,
                relatedBy: NSLayoutConstraint.Relation.equal,
                toItem: view,
                attribute: NSLayoutConstraint.Attribute.bottom,
                multiplier: 1,
                constant: 0
            )
        ])
    }
}
