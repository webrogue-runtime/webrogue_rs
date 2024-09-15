#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
#import <JavaScriptCore/JavaScriptCore.h>

#include <string>
#include <cassert>

extern "C" void webrogue_ios_main();

typedef void (*onMainCallback)(void* userdata);

extern "C" void webrogueRunOnMainThread(onMainCallback f, void* userdata) {
    dispatch_sync(dispatch_get_main_queue(), ^{
        f(userdata);
    });
}

bool checkWASMJSCSupport() {
    JSContext *context = [[JSContext alloc] init];
    JSValue *retValue = [context evaluateScript: @"typeof WebAssembly"];
    bool result = [[retValue toString] isEqualToString: @"object"];
    return result;
}

extern "C" int webrogueMain() {
    assert(checkWASMJSCSupport());
    webrogue_ios_main();
    return 0;
}
