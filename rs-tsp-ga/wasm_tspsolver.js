/* tslint:disable */
var wasm;

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function passArrayF32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getFloat32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}

const TextDecoder = require('util').TextDecoder;

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
* @param {Float32Array} arg0
* @param {Float32Array} arg1
* @param {number} arg2
* @param {number} arg3
* @returns {string}
*/
module.exports.sovle_tsp = function(arg0, arg1, arg2, arg3) {
    const [ptr0, len0] = passArrayF32ToWasm(arg0);
    const [ptr1, len1] = passArrayF32ToWasm(arg1);
    const retptr = globalArgumentPtr();
    wasm.sovle_tsp(retptr, ptr0, len0, ptr1, len1, arg2, arg3);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

};

const __wbg_random_8cdd17579946bb97_target = Math.random.bind(Math) || function() {
    throw new Error(`wasm-bindgen: Math.random.bind(Math) does not exist`);
};

module.exports.__wbg_random_8cdd17579946bb97 = function() {
    return __wbg_random_8cdd17579946bb97_target();
};

module.exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

wasm = require('./wasm_tspsolver_bg');
