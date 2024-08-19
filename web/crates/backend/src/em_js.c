#include "emscripten.h"
#include <stdint.h>

// clang-format off
EM_JS(void, wr_em_js_execFunc, (const char *funcNamePtr), {
    Asyncify.handleSleep(wakeUp => {
        Module.modsExecFinished = wakeUp;
        let funcName = UTF8ToString(funcNamePtr);
        Module.modsWorker.postMessage([ "exec", funcName ]);
    });
});
EM_JS(void, wr_em_js_readModMem, (uint32_t modPtr, uint32_t size, void *hostPtr), {
    Asyncify.handleSleep(wakeUp => {
        Module.gotMemorySlice = function (memorySlice) {
            HEAP8.set(new Int8Array(memorySlice), hostPtr);
            wakeUp();
        };
        Module.workerSharedArray[1] = BigInt(modPtr);
        Module.workerSharedArray[2] = BigInt(size);
        Atomics.store(Module.workerSharedArray, 0, BigInt(2));
        Atomics.notify(Module.workerSharedArray, 0);
    });
});
EM_JS(void, wr_em_js_writeModMem, (uint32_t modPtr, uint32_t size, const void *hostPtr), {
    Asyncify.handleSleep(wakeUp => {
        let dataToWrite = HEAP8.slice(hostPtr, hostPtr + size);
        Module.gotMemorySlice = function (memorySlice) {
            new Int8Array(memorySlice).set(dataToWrite);
            Module.sliceWrote = wakeUp;
            Atomics.store(Module.workerSharedArray, 0, BigInt(3));
            Atomics.notify(Module.workerSharedArray, 0);
        };
        Module.workerSharedArray[1] = BigInt(modPtr);
        Module.workerSharedArray[2] = BigInt(size);
        Atomics.store(Module.workerSharedArray, 0, BigInt(2));
        Atomics.notify(Module.workerSharedArray, 0);
    });
});
EM_JS(void, wr_em_js_initWasmModule, (const uint8_t *pointer, int size), {
    Asyncify.handleSleep(wakeUp => {
        Module.workerSharedBuffer = new SharedArrayBuffer(256);
        Module.workerSharedArray = new BigInt64Array(Module.workerSharedBuffer);
        var modsWasmData = HEAPU8.subarray(pointer, pointer + size);

        Module.modsWorker.onmessage = function (message) {
            let command = message.data[0];
            if (command == 0) { // instantiated
                wakeUp();
            } else if (command == 1) { // exec_finished
                Module.executionFinished = true;
        
                let modsExecFinished = Module.modsExecFinished;
                Module.modsExecFinished = undefined;
                modsExecFinished();
            } else if (command == 2) { // exec_imported
                Module.importedFuncId = message.data[1];
                Module.importedFuncArgs = message.data[2];
                Module.memorySize = message.data[3];
                Module.executionFinished = false;

                let modsExecFinished = Module.modsExecFinished;
                Module.modsExecFinished = undefined;
                modsExecFinished();
            } else if (command == 3) { // memory_slice
                Module.gotMemorySlice(message.data[1]);
            } else if (command == 4) { // memory_slice_wrote
                Module.sliceWrote();
            } else if (command = 5) { // error
                Module.modError = message.data[1];
                Module.modError = Int8Array.from(Array.from(Module.modError).map(letter => letter.charCodeAt(0)));
                // Module.modErrorStack = message.data[2];
                Module.executionFinished = true;
                
                let modsExecFinished = Module.modsExecFinished;
                Module.modsExecFinished = undefined;
                modsExecFinished();
            } else {
                console.error("host: unknown command ", command);
            }
        };
        Module.modsWorker.postMessage(["instantiate", modsWasmData, Module.importedFuncNames, Module.workerSharedBuffer, Module.sharedMemory]);
    });
});
EM_JS(void, wr_em_js_continueFuncExecution, (), {
    Asyncify.handleSleep(wakeUp => {
        Module.modsExecFinished = wakeUp;
        Atomics.store(Module.workerSharedArray, 0, BigInt(1));
        Atomics.notify(Module.workerSharedArray, 0);
    });
});
EM_JS(bool, wr_em_js_isExecutionFinished, (), {
    return Module.executionFinished;
});
EM_JS(int, wr_em_js_getImportedFuncId, (), {
    return Module.importedFuncId;
});
EM_JS(int,wr_em_js_modErrorSize, (), {
    return Module.modError ? Module.modError.length : 0
});
EM_JS(void, wr_em_js_getModError, (char *error), {
    HEAP8.set(Module.modError, error);
});
EM_JS(void, wr_em_js_makeWorker, (const char *jsonPtr), {
    Module.modsWorker = new Worker("worker.js");
    let namesJson = UTF8ToString(jsonPtr);
    Module.importedFuncNames = JSON.parse(namesJson);
    Module.modError = undefined;
});
EM_JS(void, wr_em_js_terminateWorker, (), {
    Module.modsWorker.terminate();
    delete Module.modsWorker;
    Module.executionFinished = true;
});
EM_JS(int32_t, wr_em_js_getArgInt32, (uint32_t argNum), {
    return Module.importedFuncArgs[argNum]
});
EM_JS(uint32_t, wr_em_js_getArgUInt32, (uint32_t argNum), {
    return Module.importedFuncArgs[argNum]
});
EM_JS(int64_t, wr_em_js_getArgInt64, (uint32_t argNum), {
    return Module.importedFuncArgs[argNum]
});
EM_JS(uint64_t, wr_em_js_getArgUInt64, (uint32_t argNum), {
    return Module.importedFuncArgs[argNum]
});
EM_JS(float, wr_em_js_getArgFloat, (uint32_t argNum), {
    return Module.importedFuncArgs[argNum]
});
EM_JS(double, wr_em_js_getArgDouble, (uint32_t argNum), {
    return Module.importedFuncArgs[argNum]
});
EM_JS(void, wr_em_js_writeInt32Result, (int32_t result), {
    Module.workerSharedArray[1] = BigInt(result)
});
EM_JS(void, wr_em_js_writeUInt32Result, (uint32_t result), {
    Module.workerSharedArray[1] = BigInt(result)
});
EM_JS(void, wr_em_js_writeInt64Result, (int64_t result), {
    Module.workerSharedArray[1] = result
});
EM_JS(void, wr_em_js_writeUInt64Result, (uint64_t result), {
    Module.workerSharedArray[1] = result
});
EM_JS(void, wr_em_js_writeFloatResult, (float result), {
    let buffer = new ArrayBuffer(8);
    (new Float64Array(buffer))[0] = result;
    Module.workerSharedArray[1] = (new BigInt64Array(buffer))[0];
});
EM_JS(void, wr_em_js_writeDoubleResult, (double result), {
    let buffer = new ArrayBuffer(8);
    (new Float64Array(buffer))[0] = result;
    Module.workerSharedArray[1] = (new BigInt64Array(buffer))[0];
});
EM_JS(uint32_t, wr_em_js_memorySize, (), {
    return Module.memorySize
});
EM_JS(void, wr_em_js_makeSharedMemory, (uint32_t inital_pages, uint32_t max_pages), {
    const memory = new WebAssembly.Memory({
        initial: inital_pages,
        maximum: max_pages,
        shared: true,
    });
    Module.sharedMemory = memory;
    Module.sharedMemoryBuffer = memory.buffer;
});
// clang-format on

extern void wr_rs_em_js_initWasmModule(const uint8_t *pointer, uint32_t size) {
  wr_em_js_initWasmModule(pointer, size);
}
extern void wr_rs_em_js_makeWorker(const uint8_t *jsonPtr) {
  wr_em_js_makeWorker(jsonPtr);
}
extern void wr_rs_em_js_terminateWorker() { wr_em_js_terminateWorker(); }
extern void wr_rs_em_js_execFunc(const char *funcNamePtr) {
  wr_em_js_execFunc(funcNamePtr);
}
extern bool wr_rs_em_js_isExecutionFinished() {
  return wr_em_js_isExecutionFinished();
}
extern void wr_rs_em_js_continueFuncExecution() {
  wr_em_js_continueFuncExecution();
}
extern uint32_t wr_rs_em_js_modErrorSize() { return wr_em_js_modErrorSize(); }
extern void wr_rs_em_js_getModError(char *error) {
  wr_em_js_getModError(error);
}
extern uint32_t wr_rs_em_js_getImportedFuncId() {
  return wr_em_js_getImportedFuncId();
}
extern uint32_t wr_rs_em_js_getArgU32(uint32_t argNum) {
  return wr_em_js_getArgUInt32(argNum);
}
extern void wr_rs_em_js_setResultU32(uint32_t result) {
  wr_em_js_writeUInt32Result(result);
}
extern uint64_t wr_rs_em_js_getArgU64(uint32_t argNum) {
  return wr_em_js_getArgUInt64(argNum);
}
extern void wr_rs_em_js_setResultU64(uint64_t result) {
  wr_em_js_writeUInt64Result(result);
}
extern int32_t wr_rs_em_js_getArgI32(uint32_t argNum) {
  return wr_em_js_getArgInt32(argNum);
}
extern void wr_rs_em_js_setResultI32(int32_t result) {
  wr_em_js_writeInt32Result(result);
}
extern int64_t wr_rs_em_js_getArgI64(uint32_t argNum) {
  return wr_em_js_getArgInt64(argNum);
}
extern void wr_rs_em_js_setResultI64(int64_t result) {
  wr_em_js_writeInt64Result(result);
}
extern float wr_rs_em_js_getArgF32(uint32_t argNum) {
  return wr_em_js_getArgFloat(argNum);
}
extern void wr_rs_em_js_setResultF32(float result) {
  wr_em_js_writeFloatResult(result);
}
extern double wr_rs_em_js_getArgF64(uint32_t argNum) {
  return wr_em_js_getArgDouble(argNum);
}
extern void wr_rs_em_js_setResultF64(double result) {
  wr_em_js_writeDoubleResult(result);
}
extern void wr_rs_em_js_readModMem(uint32_t modPtr, uint32_t size,
                                   char *hostPtr) {
  wr_em_js_readModMem(modPtr, size, hostPtr);
}
extern void wr_rs_em_js_writeModMem(uint32_t modPtr, uint32_t size,
                                    const char *hostPtr) {
  wr_em_js_writeModMem(modPtr, size, hostPtr);
}
extern void wr_rs_sleep(uint32_t ms) { emscripten_sleep(ms); }
extern uint32_t wr_rs_em_js_memorySize() { return wr_em_js_memorySize(); }
extern void wr_rs_em_js_makeSharedMemory(uint32_t inital_pages, uint32_t max_pages) { 
  wr_em_js_makeSharedMemory(inital_pages, max_pages); 
}
