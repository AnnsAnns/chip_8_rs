WebAssembly.instantiateStreaming(fetch("chip_8_rs.wasm"))
    .then(function (wasmModule) {
    // this saves the exported function from WASM module for use in JS
    var rust = wasmModule.instance.exports;
});
