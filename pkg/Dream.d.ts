declare namespace wasm_bindgen {
    /* tslint:disable */
    /* eslint-disable */

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly main: (a: number, b: number) => number;
    readonly __wasm_bindgen_func_elem_90835: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_90909: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_100316: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_3: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_4: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_5: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_6: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_7: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_8: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100316_9: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_100317: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_95703: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export5: (a: number, b: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_start: () => void;
}

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
