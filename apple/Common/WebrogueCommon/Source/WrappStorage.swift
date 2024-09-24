import Foundation
import UniformTypeIdentifiers
import Combine

public class WrappStorage: ObservableObject {
    public static let wrappType = UTType.init(
        exportedAs: "io.github.webrogue-runtime.wrapp"
    )

    let wrappDirectoryPath: String
    let fileManager: FileManager

    public init() {
        let fileManager = FileManager.default
        let documentDirectoryPath = fileManager.urls(
            for: .documentDirectory,
            in: .userDomainMask
        ).first!.relativePath
        let wrappDirectoryPath = documentDirectoryPath + "/.wrapp"
        if !fileManager.fileExists(atPath: wrappDirectoryPath) {
            try! fileManager.createDirectory(atPath: wrappDirectoryPath, withIntermediateDirectories: true)
        }
        self.wrappDirectoryPath = wrappDirectoryPath
        self.fileManager = fileManager
    }

    func store(_ url: URL) {
        let content = fileManager.contents(atPath: url.relativePath)!
    }
}
