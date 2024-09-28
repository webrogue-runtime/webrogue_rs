#include <cstdio>
#include <stddef.h>
#include "SDL.h"
#include "jni.h"

extern "C" void webrogue_main();

JNIEnv *jniEnv;
jclass webrogue_activity_class;
jmethodID print_bytes_method_id;

extern "C" int SDL_main(int argc, char *argv[]) {
    jniEnv = (JNIEnv *) SDL_AndroidGetJNIEnv();
    webrogue_activity_class = jniEnv->FindClass("io/github/webrogue_runtime/WebrogueActivity");
    print_bytes_method_id = jniEnv->GetStaticMethodID(
            webrogue_activity_class, "printBytes", "([B)V");
    webrogue_main();
    return 0;
}

extern "C" void webrogue_android_print(char *str, size_t len) {
    jbyteArray arg = jniEnv->NewByteArray(len);
    jbyte *bytes = jniEnv->GetByteArrayElements( arg, 0);
    memcpy(bytes, str, len);
    jniEnv->SetByteArrayRegion(arg, 0, len, bytes );
    jniEnv->ReleaseByteArrayElements( arg, bytes, 0 );
    jniEnv->CallStaticVoidMethodA(webrogue_activity_class, print_bytes_method_id, (const jvalue *)&arg);
}