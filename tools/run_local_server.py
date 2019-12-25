
from http.server import BaseHTTPRequestHandler, HTTPServer

PORT = 8080
HTML_PATH = "index.html"
JS_PATH = "../bulletforcehax/pkg/bulletforcehax.js"
WASM_PATH = "../bulletforcehax/pkg/bulletforcehax_bg.wasm"

def read_file(path):
    with open(path, 'rb') as f:
        return f.read()

class RequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/":
            content_type = "text/html; charset=utf-8"
            body = read_file(HTML_PATH)
        elif self.path.endswith(".js"):
            content_type = "text/javascript"
            body = read_file(JS_PATH)
        elif self.path.endswith(".wasm"):
            content_type = "application/wasm"
            body = read_file(WASM_PATH)
        else:
            self.send_response(404)
            self.send_header('Content-Type', 'text/html')
            self.end_headers()
            self.wfile.write(bytes("404 Not Found", "utf8"))
            return

        # Send response status code
        self.send_response(200)
        self.send_header('Content-Type', content_type)
        self.end_headers()

        self.wfile.write(body)

print('starting server...')
server = HTTPServer(('127.0.0.1', PORT), RequestHandler)
server.serve_forever()
