import * as wasm from "./pizza_wasm_bg.wasm";
export * from "./pizza_wasm_bg.js";
import { __wbg_set_wasm } from "./pizza_wasm_bg.js";
__wbg_set_wasm(wasm);