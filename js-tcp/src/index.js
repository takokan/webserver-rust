const net = require('net');

const server = net.createServer((socket) => {
    console.log("client connected");

    socket.on('data', (data) => {
        console.log('Received:', data.toString());
        socket.write(`server echo: ${data}`);
    })

    socket.end('end', () => {
        console.log("Client Disconnected");
    })

    socket.on('error', (err) => {
        console.log('socket error', err);
    })

    socket.write('Welcome to the TCP server!\n');
});


server.on('error', (err) => {
    console.error("Server error: ", err);
});

const PORT = 8080;
const HOST = '127.0.0.1';

server.listen(PORT, HOST, () => {
    console.log(`Server listening on ${HOST}:${PORT}`);
});
