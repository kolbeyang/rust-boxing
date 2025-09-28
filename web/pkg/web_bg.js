let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(
state => {
    wasm.__wbindgen_export_5.get(state.dtor)(state.a, state.b);
}
);

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_5.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}
/**
 * @returns {FighterWeb[]}
 */
export function get_fighters() {
    const ret = wasm.get_fighters();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

export function greet() {
    wasm.greet();
}

function __wbg_adapter_6(arg0, arg1, arg2) {
    wasm.closure267_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_66(arg0, arg1, arg2, arg3) {
    wasm.closure301_externref_shim(arg0, arg1, arg2, arg3);
}

/**
 * @enum {0 | 1 | 2}
 */
export const FistStateWeb = Object.freeze({
    Resting: 0, "0": "Resting",
    Extending: 1, "1": "Extending",
    Retracting: 2, "2": "Retracting",
});

const FighterWebFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_fighterweb_free(ptr >>> 0, 1));

export class FighterWeb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FighterWeb.prototype);
        obj.__wbg_ptr = ptr;
        FighterWebFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FighterWebFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_fighterweb_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get number() {
        const ret = wasm.__wbg_get_fighterweb_number(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set number(arg0) {
        wasm.__wbg_set_fighterweb_number(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.fighterweb_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get description() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.fighterweb_description(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get color() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.fighterweb_color(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) FighterWeb.prototype[Symbol.dispose] = FighterWeb.prototype.free;

const FistWebFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_fistweb_free(ptr >>> 0, 1));

export class FistWeb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FistWeb.prototype);
        obj.__wbg_ptr = ptr;
        FistWebFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FistWebFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_fistweb_free(ptr, 0);
    }
    /**
     * @returns {Point}
     */
    get position() {
        const ret = wasm.__wbg_get_fistweb_position(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set position(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_fistweb_position(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {FistStateWeb}
     */
    get state() {
        const ret = wasm.__wbg_get_fistweb_state(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {FistStateWeb} arg0
     */
    set state(arg0) {
        wasm.__wbg_set_fistweb_state(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) FistWeb.prototype[Symbol.dispose] = FistWeb.prototype.free;

const GameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_game_free(ptr >>> 0, 1));

export class Game {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Game.prototype);
        obj.__wbg_ptr = ptr;
        GameFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_game_free(ptr, 0);
    }
    /**
     * @param {number} player0_number
     * @param {number} player1_number
     * @returns {Promise<Game>}
     */
    static new(player0_number, player1_number) {
        const ret = wasm.game_new(player0_number, player1_number);
        return ret;
    }
    /**
     * @returns {GameStateWeb}
     */
    step() {
        const ret = wasm.game_step(this.__wbg_ptr);
        return GameStateWeb.__wrap(ret);
    }
}
if (Symbol.dispose) Game.prototype[Symbol.dispose] = Game.prototype.free;

const GameStateWebFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gamestateweb_free(ptr >>> 0, 1));

export class GameStateWeb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GameStateWeb.prototype);
        obj.__wbg_ptr = ptr;
        GameStateWebFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameStateWebFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gamestateweb_free(ptr, 0);
    }
    /**
     * @returns {PlayerWeb}
     */
    get player_0() {
        const ret = wasm.__wbg_get_gamestateweb_player_0(this.__wbg_ptr);
        return PlayerWeb.__wrap(ret);
    }
    /**
     * @param {PlayerWeb} arg0
     */
    set player_0(arg0) {
        _assertClass(arg0, PlayerWeb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_gamestateweb_player_0(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {PlayerWeb}
     */
    get player_1() {
        const ret = wasm.__wbg_get_gamestateweb_player_1(this.__wbg_ptr);
        return PlayerWeb.__wrap(ret);
    }
    /**
     * @param {PlayerWeb} arg0
     */
    set player_1(arg0) {
        _assertClass(arg0, PlayerWeb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_gamestateweb_player_1(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {boolean}
     */
    get is_done() {
        const ret = wasm.__wbg_get_gamestateweb_is_done(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set is_done(arg0) {
        wasm.__wbg_set_gamestateweb_is_done(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) GameStateWeb.prototype[Symbol.dispose] = GameStateWeb.prototype.free;

const PlayerWebFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_playerweb_free(ptr >>> 0, 1));

export class PlayerWeb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PlayerWeb.prototype);
        obj.__wbg_ptr = ptr;
        PlayerWebFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PlayerWebFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_playerweb_free(ptr, 0);
    }
    /**
     * @returns {Point}
     */
    get position() {
        const ret = wasm.__wbg_get_playerweb_position(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set position(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_playerweb_position(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get rotation() {
        const ret = wasm.__wbg_get_playerweb_rotation(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set rotation(arg0) {
        wasm.__wbg_set_playerweb_rotation(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Point}
     */
    get velocity() {
        const ret = wasm.__wbg_get_playerweb_velocity(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set velocity(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_playerweb_velocity(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get health() {
        const ret = wasm.__wbg_get_playerweb_health(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set health(arg0) {
        wasm.__wbg_set_playerweb_health(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get energy() {
        const ret = wasm.__wbg_get_playerweb_energy(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set energy(arg0) {
        wasm.__wbg_set_playerweb_energy(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {FistWeb}
     */
    get fist_0() {
        const ret = wasm.__wbg_get_playerweb_fist_0(this.__wbg_ptr);
        return FistWeb.__wrap(ret);
    }
    /**
     * @param {FistWeb} arg0
     */
    set fist_0(arg0) {
        _assertClass(arg0, FistWeb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_playerweb_fist_0(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {FistWeb}
     */
    get fist_1() {
        const ret = wasm.__wbg_get_playerweb_fist_1(this.__wbg_ptr);
        return FistWeb.__wrap(ret);
    }
    /**
     * @param {FistWeb} arg0
     */
    set fist_1(arg0) {
        _assertClass(arg0, FistWeb);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_playerweb_fist_1(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Control}
     */
    get last_control() {
        const ret = wasm.__wbg_get_playerweb_last_control(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Control} arg0
     */
    set last_control(arg0) {
        wasm.__wbg_set_playerweb_last_control(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) PlayerWeb.prototype[Symbol.dispose] = PlayerWeb.prototype.free;

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr >>> 0, 1));

export class Point {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_point_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export function __wbg_alert_ba51d1622293fc6b(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

export function __wbg_call_13410aac570ffff7() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_call_a5400b25a865cfd8() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_fighterweb_new(arg0) {
    const ret = FighterWeb.__wrap(arg0);
    return ret;
};

export function __wbg_game_new(arg0) {
    const ret = Game.__wrap(arg0);
    return ret;
};

export function __wbg_getRandomValues_3c9c0d586e575a16() { return handleError(function (arg0, arg1) {
    globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
}, arguments) };

export function __wbg_new_2e3c58a15f39f5f9(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_66(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return ret;
    } finally {
        state0.a = state0.b = 0;
    }
};

export function __wbg_newnoargs_254190557c45b4ec(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_parse_442f5ba02e5eaf8b() { return handleError(function (arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return ret;
}, arguments) };

export function __wbg_queueMicrotask_25d0739ac89e8c88(arg0) {
    queueMicrotask(arg0);
};

export function __wbg_queueMicrotask_4488407636f5bf24(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
};

export function __wbg_resolve_4055c623acdd6a1b(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
};

export function __wbg_static_accessor_GLOBAL_8921f820c2ce3f12() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_GLOBAL_THIS_f0a4409105898184() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_SELF_995b214ae681ff99() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_WINDOW_cde3890479c675ea() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_stringify_b98c93d0a190446a() { return handleError(function (arg0) {
    const ret = JSON.stringify(arg0);
    return ret;
}, arguments) };

export function __wbg_then_e22500defe16819f(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
};

export function __wbg_wbindgencbdrop_eb10308566512b88(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export function __wbg_wbindgenisfunction_8cee7dce3725ae74(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

export function __wbg_wbindgenisundefined_c4b71d073b92f3c5(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbg_wbindgenstringget_0f16a6ddddef376f(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_wbindgenthrow_451ec1a8469d7eb6(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_cast_f456093041beed15(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 266, function: Function { arguments: [Externref], shim_idx: 267, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, 266, __wbg_adapter_6);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

