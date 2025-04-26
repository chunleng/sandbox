const ports = [];

onconnect = function (event) {
  const port = event.ports[0];

  port.onmessage = function (e) {
    ports.forEach(p => p.postMessage(e.data));
  };

  ports.push(port);
};
