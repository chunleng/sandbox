async function init_worker() {
    self.onmessage = async event => {
        self.postMessage(event.data);
    };
};

init_worker();
