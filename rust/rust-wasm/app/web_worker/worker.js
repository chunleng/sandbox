// The worker has its own scope and no direct access to functions/objects of the global scope. We import the generated
// JS file to make `wasm_bindgen` available which we need to initialize our Wasm code.
importScripts('./pkg/wasm_web_worker.js');

console.log('Initializing worker')

const {validate} = wasm_bindgen;

async function init_wasm_in_worker() {
    // Load the Wasm file by awaiting the Promise returned by `wasm_bindgen`.
    await wasm_bindgen('./pkg/wasm_web_worker_bg.wasm');

    // Set callback to handle messages passed to the worker.
    self.onmessage = async event => {
        const send_value = event.data;
        if (validate(send_value)) {
            // Send response back to be handled by callback in main thread.
            self.postMessage(event.data);
        }
    };
};

init_wasm_in_worker();
