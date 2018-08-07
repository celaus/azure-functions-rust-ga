
const path = require('path').join(__dirname, 'wasm_tspsolver_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./wasm_tspsolver'] = require('./wasm_tspsolver');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
