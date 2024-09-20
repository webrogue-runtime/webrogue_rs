//
//  ContentView.swift
//  launcher
//
//  Created by Artem on 20.09.24.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
            Button(action: {
                var pathComponents = Bundle.main.executablePath!.components(separatedBy: "/")
                pathComponents.removeLast()
                pathComponents.append("webrogue_runtime")


                let task = Process()
                let pipe = Pipe()

                task.standardOutput = pipe
                task.standardError = pipe
                task.arguments = []
                task.launchPath = "/"+pathComponents.joined(separator: "/")
                task.standardInput = nil
                task.launch()

                DispatchQueue.global().async {
                    let data = pipe.fileHandleForReading.readDataToEndOfFile()
                    let output = String(data: data, encoding: .utf8)!
                    print(output)
                }

            }, label: {
                Text("Hello, world!")
            })
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
