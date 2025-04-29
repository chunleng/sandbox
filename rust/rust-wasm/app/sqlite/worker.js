// The worker has its own scope and no direct access to functions/objects of the global scope. We import the generated
// JS file to make `wasm_bindgen` available which we need to initialize our Wasm code.
import init, {addAndCount} from './pkg/wasm_sqlite.js';

console.log('Initializing worker')

async function init_wasm_in_worker() {
    await init();
    // Set callback to handle messages passed to the worker.
    self.onmessage = async event => {
        self.postMessage(await addAndCount());
    };
};

init_wasm_in_worker();
