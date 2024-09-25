import SwiftUI
import WebrogueCommon
import UniformTypeIdentifiers

struct WebrogueView: View {
    @StateObject var stoarge = WrappStorage()
    @State var isFileImporterPresented = false

    var body: some View {
        NavigationView {
            Group {
                List(stoarge.refs, id: \.metadata.sha256) { ref in
                    NavigationLink {
                        WrappRefView(ref: ref)
                    } label: {
                        Text(ref.metadata.sha256)
                    }
                }

            }
                .navigationBarTitleDisplayMode(.inline)
                .navigationTitle("webrogue")
                .toolbar {
                    Button("Add") {
                        isFileImporterPresented = true
                    }
                }
                .fileImporter(
                    isPresented: $isFileImporterPresented,
                    allowedContentTypes: [
                        UTType.data,
                        WrappStorage.wrappType,
                    ]
                ) { result in
                    switch result {
                    case .success(let url):
                        stoarge.store(url)
                    case .failure(_):
                        break
                    }
                }
        }
    }
}

#Preview {
    WebrogueView()
}
