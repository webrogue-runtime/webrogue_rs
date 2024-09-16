import SwiftUI

struct WebrogueView: View {
    var body: some View {
        NavigationView {
            Text("SwiftUI")
                .navigationBarTitleDisplayMode(.inline)
                .navigationTitle("webrogue")
                .toolbar {
                    Button("Run") {
                        WebrogueAppDelegate.shared?.runGame() { _ in }
                    }
                }
        }
    }
}
