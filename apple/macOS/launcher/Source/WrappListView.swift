//
//  ContentView.swift
//  launcher
//
//  Created by Artem on 20.09.24.
//

import SwiftUI
import Wrappman
import UniformTypeIdentifiers

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
                Storage.wrappType
            ]
        ) { result in
            switch result {
            case .success(let url):
                Storage.store(url)
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
