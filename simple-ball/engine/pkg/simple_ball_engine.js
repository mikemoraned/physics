
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
/**
*/
export class Dimension {

    static __wrap(ptr) {
        const obj = Object.create(Dimension.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dimension_free(ptr);
    }
    /**
    * @returns {number}
    */
    get side_length() {
        const ret = wasm.__wbg_get_dimension_side_length(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set side_length(arg0) {
        wasm.__wbg_set_dimension_side_length(this.ptr, arg0);
    }
}
/**
*/
export class Screen {

    static __wrap(ptr) {
        const obj = Object.create(Screen.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_screen_free(ptr);
    }
    /**
    * @returns {Dimension}
    */
    get dimension() {
        const ret = wasm.__wbg_get_screen_dimension(this.ptr);
        return Dimension.__wrap(ret);
    }
    /**
    * @param {Dimension} arg0
    */
    set dimension(arg0) {
        _assertClass(arg0, Dimension);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        wasm.__wbg_set_screen_dimension(this.ptr, ptr0);
    }
    /**
    * @param {number} side_length
    */
    constructor(side_length) {
        const ret = wasm.screen_new(side_length);
        return Screen.__wrap(ret);
    }
}
/**
*/
export class Simulation {

    static __wrap(ptr) {
        const obj = Object.create(Simulation.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simulation_free(ptr);
    }
    /**
    * @param {number} num_balls
    * @param {Terrain} terrain
    * @param {Screen} screen
    */
    constructor(num_balls, terrain, screen) {
        _assertClass(terrain, Terrain);
        _assertClass(screen, Screen);
        const ret = wasm.simulation_new(num_balls, terrain.ptr, screen.ptr);
        return Simulation.__wrap(ret);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    set_force(x, y) {
        wasm.simulation_set_force(this.ptr, x, y);
    }
    /**
    * @param {Function} iter_fn
    */
    iter_ball_positions(iter_fn) {
        try {
            wasm.simulation_iter_ball_positions(this.ptr, addBorrowedObject(iter_fn));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {number} elapsed_since_last_update
    */
    update(elapsed_since_last_update) {
        wasm.simulation_update(this.ptr, elapsed_since_last_update);
    }
}
/**
*/
export class Terrain {

    static __wrap(ptr) {
        const obj = Object.create(Terrain.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_terrain_free(ptr);
    }
    /**
    * @returns {number}
    */
    get width() {
        const ret = wasm.__wbg_get_terrain_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        wasm.__wbg_set_terrain_width(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get height() {
        const ret = wasm.__wbg_get_terrain_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_terrain_height(this.ptr, arg0);
    }
    /**
    * @param {Uint8Array} data
    * @returns {Terrain}
    */
    static from_png_terrain_image(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.terrain_from_png_terrain_image(ptr0, len0);
        return Terrain.__wrap(ret);
    }
    /**
    * @returns {Terrain}
    */
    halfed() {
        const ret = wasm.terrain_halfed(this.ptr);
        return Terrain.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    as_grayscale_height_image() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.terrain_as_grayscale_height_image(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_63d19a8aab427726 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_call_a51357fb7467f969 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_random_5ee0189319837e3a = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();


    return wasm;
}

function initSync(bytes) {
    const imports = getImports();

    initMemory(imports);

    const module = new WebAssembly.Module(bytes);
    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('simple_ball_engine_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
