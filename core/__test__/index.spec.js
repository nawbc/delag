const Delag = require('../dist');

const server = new Delag.Server();

server.on('request', (req, res) => {
  res.end('Hello');
});

server.listen(8080, '127.0.0.1');
