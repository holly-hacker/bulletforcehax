import json as json_lib
import os
import re
import sys

import requests

# find json link of latest version
game_url = "https://www.crazygames.com/game/bullet-force-multiplayer"
iframe_pattern = re.escape("https://www.crazygames.com/gameframe/bullet-force-multiplayer/CHANGE_ME/index.html").replace("CHANGE_ME", "\\d+")
game_source = requests.get(game_url).text

match = re.search(iframe_pattern, game_source)
iframe_url = match.group()
print("iframe url:     " + iframe_url)

json_pattern = re.escape("https://files.crazygames.com/bullet-force-multiplayer/") + "[^\"]+"
iframe_source = requests.get(iframe_url).text
match = re.search(json_pattern, iframe_source)
json_url = match.group()
print("config url:     " + json_url)

# print loader version too, just in case
print("loader version: " + re.search("loader: \"([^\"]+)\"", iframe_source).group(1) + " (expecting 5.6.x)")

# get info from json file
json = requests.get(json_url).json()

ver = json["productVersion"]
unity_ver = json["unityVersion"]

print("game version:   %s (Unity %s)" % (ver, unity_ver))
download_folder = os.path.join('game_bins', 'v' + ver)

if os.path.exists(download_folder): # TODO: implement better check? eg. are all files downloaded
    print("Version already downloaded, exiting")
    quit()

print("New version!")
os.makedirs(download_folder)

# TODO: check if wasm or asm.js (going back to asm.js is unlikely though)
data_url = json["dataUrl"]
code_url = json["wasmCodeUrl"]
framework_url = json["wasmFrameworkUrl"]

json_path = json_url.split('/')[-1]
url_base = json_url.replace(json_path, "")
json_path = json_path.split('?')[0]

# https://stackoverflow.com/a/15645088
def downloadToPath(file_name):
    url = url_base + file_name
    download_path = os.path.join(download_folder, file_name)
    with open(download_path, "wb") as f:
        response = requests.get(url, stream=True)
        total_length = response.headers.get('content-length')

        if total_length is None: # no content length header
            f.write(response.content)
        else:
            dl = 0
            total_length = int(total_length)
            for data in response.iter_content(chunk_size=4096):
                dl += len(data)
                f.write(data)
                done = int(50 * dl / total_length)
                sys.stdout.write("\r[%s%s] Downloading %s" % ('=' * done, ' ' * (50-done), file_name) )
                sys.stdout.flush()
            print()

# finally write files to disk
print('Downloading data files...')
downloadToPath(code_url)
downloadToPath(framework_url)
downloadToPath(data_url)

# optionally rewrite json here
json_filename = "manifest.json"
json_string = json_lib.dumps(json, indent=4)
json_bytes = bytes(json_string, 'utf-8')
download_path = os.path.join(download_folder, json_filename)
with open(download_path, 'wb') as f:
    f.write(json_bytes)
