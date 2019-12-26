
from http.server import BaseHTTPRequestHandler, HTTPServer
import os

PORT = 8080
HTML_PATH = "index.html"
JS_PATH = "../bulletforcehax/pkg/bulletforcehax.js"
WASM_PATH = "../bulletforcehax/pkg/bulletforcehax_bg.wasm"
GAME_BASE_PATH = "game_bins/v1.70.0" # TODO: make this not hardcoded

def read_file(path):
    with open(path, 'rb') as f:
        return f.read()

class RequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        # decide content type
        if self.path.endswith(".js"):
            content_type = "text/javascript"
        elif self.path.endswith(".json"):
            content_type = "application/json"
        elif self.path.endswith(".wasm"):
            content_type = "application/wasm"
            pass
        elif self.path.endswith(".unityweb"):
            content_type = "application/vnd.unity"
        else:
            content_type = "text/html; charset=utf-8"

        # decode content
        if self.path == "/":
            body = read_file(HTML_PATH)
        elif self.path.startswith("/Build/"): # using this to remain compatible with CrazyGames UnityLoader
            body = read_file(GAME_BASE_PATH + self.path.replace("/Build/", "/"))
        elif self.path.endswith("hax.js"):
            body = read_file(JS_PATH)
        elif self.path.endswith("hax_bg.wasm"):
            body = read_file(WASM_PATH)
        elif os.path.exists(self.path):
            body = read_file(self.path)
        else:
            self.send_response(404)
            self.send_header('Content-Type', 'text/html')
            self.end_headers()
            self.wfile.write(bytes("404 Not Found", "utf8"))
            return

        # Send response status code
        self.send_response(200)
        if "content_type" in locals():
            self.send_header('Content-Type', content_type)
        self.end_headers()

        self.wfile.write(body)

print('starting server...')
server = HTTPServer(('127.0.0.1', PORT), RequestHandler)
server.serve_forever()
