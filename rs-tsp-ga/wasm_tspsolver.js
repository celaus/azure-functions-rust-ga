/* tslint:disable */
var wasm;
var random = Math.random;
module.exports.__wbg_random_7c0f10165d552a04 = function() {
    return random();
};

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
/**
* @param {Float32Array} arg0
* @param {Float32Array} arg1
* @returns {TSPSolution}
*/
module.exports.sovle_tsp = function(arg0, arg1) {
    const [ptr0, len0] = passArrayF32ToWasm(arg0);
    const [ptr1, len1] = passArrayF32ToWasm(arg1);
    return TSPSolution.__construct(wasm.sovle_tsp(ptr0, len0, ptr1, len1));
};

const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

let slab_next = slab.length;

function addHeapObject(obj) {
    if (slab_next === slab.length) slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];
    
    slab_next = next;
    
    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

module.exports.__wbg_static_accessor_this_this = function() {
    return addHeapObject(this);
};

const __wbg_self_30aa89e143879306_target = function() {
    return this.self;
}  ;

const stack = [];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];
        
        return val.obj;
        
    }
}

module.exports.__wbg_self_30aa89e143879306 = function(arg0) {
    return addHeapObject(__wbg_self_30aa89e143879306_target.call(getObject(arg0)));
};

const __wbg_crypto_05f4d71036ff816d_target = function() {
    return this.crypto;
}  ;

module.exports.__wbg_crypto_05f4d71036ff816d = function(arg0) {
    return addHeapObject(__wbg_crypto_05f4d71036ff816d_target.call(getObject(arg0)));
};

const __wbg_getRandomValues_ed780990e1cb1682_target = function() {
    return this.getRandomValues;
}  ;

module.exports.__wbg_getRandomValues_ed780990e1cb1682 = function(arg0) {
    return addHeapObject(__wbg_getRandomValues_ed780990e1cb1682_target.call(getObject(arg0)));
};

const TextDecoder = require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

module.exports.__wbg_require_32e33783d10525d3 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(require(varg0));
};
/**
*/
class TSPSolution {
    
    static __construct(ptr) {
        return new TSPSolution(ptr);
    }
    
    constructor(ptr) {
        this.ptr = ptr;
    }
    
    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        wasm.__wbg_tspsolution_free(ptr);
    }
}
module.exports.TSPSolution = TSPSolution;

function dropRef(idx) {
    
    idx = idx >> 1;
    if (idx < 4) return;
    let obj = slab[idx];
    
    obj.cnt -= 1;
    if (obj.cnt > 0) return;
    
    // If we hit 0 then free up our space in the slab
    slab[idx] = slab_next;
    slab_next = idx;
}

module.exports.__wbindgen_object_drop_ref = function(i) {
    dropRef(i);
};

module.exports.__wbindgen_is_undefined = function(idx) {
    return getObject(idx) === undefined ? 1 : 0;
};

module.exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

wasm = require('./wasm_tspsolver_bg');
