const http = require('http');

let totalCount = 0;
let okCount = 0;
let errorCount = 0;

const server = http.createServer((req, res) => {
    totalCount++;

    if (req.url === '/ok') {
        okCount++;
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
            message: 'OK',
            totalCount,
            okCount,
            errorCount,
        }));
    } else if (req.url === '/error') {
        errorCount++;
        res.writeHead(500, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
            message: 'ERROR',
            totalCount,
            okCount,
            errorCount,
        }));
    } else {
        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
            message: 'Not Found',
            totalCount,
            okCount,
            errorCount,
        }));
    }

    console.log(`Total=${totalCount} OK=${okCount} ERROR=${errorCount} ${req.method} ${req.url}`);
});

const PORT = 3000;
server.listen(PORT, () => {
    console.log(`Test server running on http://localhost:${PORT}`);
    console.log(`  /ok    -> 200`);
    console.log(`  /error -> 500`);
});
