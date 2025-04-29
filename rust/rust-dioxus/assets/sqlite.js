import init, {addAndCount} from '/assets/sqlite/wasm_sqlite.js';

console.log('Initializing worker')

async function init_wasm_in_worker() {
    await init();

    // Set callback to handle messages passed to the worker.
    self.onmessage = async event => {
        self.postMessage(await addAndCount());
    };
};

init_wasm_in_worker();
