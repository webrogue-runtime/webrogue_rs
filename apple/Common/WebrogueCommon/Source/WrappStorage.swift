import Foundation
import UniformTypeIdentifiers
import Combine

public final class WrappStorage: ObservableObject {
    public static let wrappType = UTType.init(
        exportedAs: "io.github.webrogue-runtime.wrapp"
    )

    let wrappDirectoryPath: String
    let fileManager: FileManager

    @Published public var refs: [WrappRef]

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
        self.refs = []

        updateRefs()
    }

    private func updateRefs() {
        refs.removeAll()
        let fileNames = try! fileManager.contentsOfDirectory(atPath: wrappDirectoryPath)
        for fileName in fileNames {
            let filePath = wrappDirectoryPath + "/" + fileName
            guard
                let content = fileManager.contents(atPath: filePath),
                let metadata = WrappMetadata(data: content)
            else { continue }
            refs.append(WrappRef(path: filePath, metadata: metadata))
        }
    }

    public func store(_ url: URL) -> WrappRef? {
        guard
            let content = fileManager.contents(atPath: url.relativePath),
            let metadata = WrappMetadata(data: content)
        else { return nil }
        let newPath = wrappDirectoryPath + "/" + metadata.sha256
        guard
            fileManager.createFile(atPath: newPath, contents: content)
        else { return nil }

        updateRefs()
        return refs.first { $0.metadata.sha256 == metadata.sha256 }
     }
}
