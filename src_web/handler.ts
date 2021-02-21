
WebAssembly.instantiateStreaming(fetch("chip_8_rs.wasm"))
.then(wasmModule => {
    // this saves the exported function from WASM module for use in JS
    let rust = wasmModule.instance.exports;
});
