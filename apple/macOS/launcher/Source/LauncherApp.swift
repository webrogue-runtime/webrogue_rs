import SwiftUI
import WebrogueCommon

@main
struct LauncherApp: App {
    @StateObject var wrappStorage = WrappStorage()

    var documentGroup: some Scene {
        let result = DocumentGroup(viewing: WrappDocument.self) { file in
            WrappRefView(
                for: WrappRef(
                    path: file.fileURL!.relativePath,
                    metadata: file.document.metadata
                )
            )
        }

        if #available(macOS 13, *) {
            return result.commandsRemoved()
        } else {
            return result
                .commands {
                    CommandGroup(replacing: .saveItem) {}
                }
        }
    }

    var body: some Scene {
//        WindowGroup {
//            WrappListView()
//        }
        documentGroup
            .commands {
                CommandGroup(replacing: .undoRedo) {}
            }
    }
}
