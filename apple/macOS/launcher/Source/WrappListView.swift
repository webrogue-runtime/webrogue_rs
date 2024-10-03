import SwiftUI
import WebrogueCommon

struct WrappListView: View {
    @State var isFileImporterPresented = false

    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
            Button(action: {
                isFileImporterPresented = true
            }, label: {
                Text("Hello, world!")
            })
        }
        .fileImporter(
            isPresented: $isFileImporterPresented,
            allowedContentTypes: [
                WrappStorage.wrappType
            ]
        ) { result in
            switch result {
            case .success(let url):
//                WrappStorage.store(url)
                break
            case .failure(_):
                break
            }
        }
        .padding()
    }
}

#Preview {
    WrappListView()
}
