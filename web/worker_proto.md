Webrogue executes WRAPP's WebAssembly code in separate worker. 
This code executes synchronously, while webrogue itself is asynchronous to make page updates possible.
webrogue and worker communicate mostly using common SharedArrayBuffer.

# Messages that can be sent to worker
## Initialization. 
This message's body is the following array: 
`["instantiate", modsWasmData, importedFuncNames, workerSharedBuffer, sharedMemory]`. 
Where:
- `modsWasmData` is a WebAssembly executable code.
- `importedFuncNames` is a JSON string containing information about imported functions.
- `workerSharedBuffer` is a SharedArrayBuffer object that will be used for communication
- `sharedMemory` may contain WebAssembly.Memory object if WRAPP needs shared memory.

# Execution
This message's body is the following array: 
`[ "exec", funcName ]`
Where:
- `funcName` is a string containing a name of function to execute (usually "_start")

# Atomic communication
Since worker executes code synchronously, it will not be able to receive messages sent from webrogue while an imported function is invoked.
Webrogue and worker are using workerSharedBuffer and Atomics API to communicate.
WorkerSharedBuffer viewed by BigInt64Array named workerSharedArray, of `wSA` for short.

## Request number
`wSA[0]` is named `Request number` it is initially set to 0. 
Webrogue may set it following values:
- `wSA[0] = 1` means return from imported function. 
Returned value is stored in `wSA[2]` if it's type is not void.
- `wSA[0] = 2` means request to read a slice of guest's memory.
Slice start address and size are stored in `wSA[2]` and `wSA[3]` respectively.
Result must be stored in workerSharedBuffer with offset of 32 bytes at start.
- `wSA[0] = 3` means request to write a slice to guest's memory.
Slice start address and size are stored in `wSA[2]` and `wSA[3]` respectively.
Slice's data is stored in workerSharedBuffer with offset of 32 bytes at start.

Worker must set `Request number` back to 0 at the end of any request.

## Status number
`wSA[1]` is named `Status number` it is initially set to 0. 
Worker may set it following values:
- `wSA[1] = 1` means need to execute an imported function. 
`wSA[2]` corresponds to `importedFuncNames`'s function index.
`wSA[3]` is size of guest's memory in bytes.
An imported function parameter is stored at `wSA[3+i]` where is is index of a parameter (starts form 0).
- `wSA[1] = 2` means that code execution is ended. 

Webrogue must set `Status number` back to 0 at the end of any request.