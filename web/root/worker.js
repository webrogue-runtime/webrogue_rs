let modsModule
let wSA
let getMemory

onmessage = function (message) {
    let command = message.data[0]
    if (command === "instantiate") {
        let modsWasmData = message.data[1]
        let importFuncNames = message.data[2]
        wSA = new BigInt64Array(message.data[3]);
        let sharedMemory = message.data[4];
        let importObject = {};
        for (const [importModuleName, importedFuncs] of Object.entries(importFuncNames)) {
            let importModule = {}
            for (const [funcName, funcDetails] of Object.entries(importedFuncs)) {
                const retType = funcDetails.ret_type
                const funcId = funcDetails.func_id;
                importModule[funcName] = function (...args) {
                    Atomics.store(wSA, 2, BigInt(funcId))
                    Atomics.store(wSA, 3, BigInt(getMemory().byteLength))
                    Atomics.store(wSA, 1, BigInt(1))
                    Atomics.notify(wSA, 1)
                    while (true) {
                        Atomics.wait(wSA, 0, BigInt(0));
                        let requestNumber = wSA[0];
                        if (requestNumber == 1) {
                            Atomics.store(wSA, 0, BigInt(0));
                            // TODO try to omit notify
                            Atomics.notify(wSA, 0, BigInt(0));
                            break;
                        } else if (requestNumber == 2) {
                            let modPtr = Number(wSA[2]);
                            let size = Number(wSA[3]);
                            let shared_slice = new Uint8Array(wSA).slice(32, 32 + size);
                            let memory_slice = new Uint8Array(getMemory().slice(modPtr, modPtr + size));
                            (shared_slice).set(memory_slice);
                        } else if (wSA[0] == 3) {
                            let modPtr = Number(wSA[2]);
                            let size = Number(wSA[3]);
                            let shared_slice = new Uint8Array(wSA).slice(32, 32 + size);
                            let memory_slice = new Uint8Array(getMemory().slice(modPtr, modPtr + size));
                            (memory_slice).set(shared_slice);
                        } else {
                            console.error("worker: unknown buffer command: ", wSA[0])
                        }
                        Atomics.store(wSA, 0, BigInt(0));
                        Atomics.notify(wSA, 0, BigInt(0));
                    }
                    let result
                    if (retType == "void") {
                        result = undefined
                    } else if (retType == "int32_t" || retType == "uint32_t") {
                        result = Number(wSA[2])
                    } else if (retType == "int64_t" || retType == "uint64_t") {
                        result = wSA[2]
                    } else if (retType == "float" || retType == "double") {
                        let buffer = new ArrayBuffer(8);
                        (new BigInt64Array(buffer))[0] = wSA[2];
                        result = (new Float64Array(buffer))[0];
                    } else {
                        console.error("unknown retType: ", retType)
                    }
                    return result
                }
            }
            importObject[importModuleName] = importModule
            if (sharedMemory) {
                if (!importObject["env"]) {
                    importObject["env"] = {};
                }
                importObject["env"]["memory"] = sharedMemory;
            }
        }

        WebAssembly.instantiate(modsWasmData, importObject).then((newModule) => {
            modsModule = newModule;
            getMemory = function () { return modsModule.instance.exports.memory.buffer }
            // TODO omit
            postMessage(["instantiated"]);
        });
    } else if (command === "exec") {
        let funcName = message.data[1]
        let error_string, error_stack
        try {
            modsModule.instance.exports[funcName]();
        } catch (error) {
            error_string = error.toString();
            error_stack = error.stack
        }
        postMessage(["exec_finished", error_string, error_stack])
    } else {
        console.error("worker: unknown command ", command)
    }
}
