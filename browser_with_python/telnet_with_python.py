import socket
import zlib

scheme = 'http://'
port = 80
http_version = 1.0


def request(url):
    s = socket.socket(
        family=socket.AF_INET,
        type=socket.SOCK_STREAM,
        proto=socket.IPPROTO_TCP
    )

    assert url.startswith(scheme)
    url = url[len(scheme):]

    if ('/' in url):
        host, path = url.split('/', 1)
    else:
        host = url
        path = ""
    path = "/"+path

    s.connect((host, port))
    s.send("GET {} HTTP/{}\r\n".format(path, http_version).encode("utf8") +
           #"Accept-Encoding: gzip\r\n".encode("utf8") +
           "Host: {}\r\n\r\n".format(host).encode("utf8"))

    # To read the response, you’d generally use the read function on sockets, which gives whatever bits of the response have already arrived.
    # Then you write a loop that collects bits of the response as they arrive.
    # However, in Python you can use the makefile helper function, which hides the loop

    response = s.makefile("r", encoding="utf8", newline="\r\n")
    # Here makefile returns a file-like object containing every byte we receive from the server

    print(response)

    statusline = response.readline()
    print(statusline)
    version, status, explanation = statusline.split(" ", 2)
    assert status == "200", "{}, {}, {}".format(version, status, explanation)

    headers = {}

    while (True):
        currHeader = response.readline()
        header, value = currHeader.split(":", 1)
        value = value.strip()
        headers[header.lower()] = value
        if (header == 'Connection' and value == 'close'):
            break
    print(headers)

    # Headers can describe all sorts of information,
    # but a couple of headers are especially important because they tell us that the data we’re trying to access is being sent in an unusual way.
    # Let’s make sure none of those are present

    assert "transfer-encoding" not in headers
    assert "content-encoding" not in headers

    body = response.read()
    s.close()
    # print(body)
    if('Content-Encoding' in headers and headers['Content-Encoding']=='gzip'):
        body = zlib.decompress(body)#.decode("utf8")
    return headers, body


url = 'http://example.org'

headers, body = request(url)
print(headers, body)
