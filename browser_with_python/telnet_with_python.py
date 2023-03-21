import socket
import zlib
import re
import ssl

http_version = 1.0


def request(url):
    # valid url
    assert '://' in url, 'invalid url'

    url = url.lower()

    s = socket.socket(
        family=socket.AF_INET,
        type=socket.SOCK_STREAM,
        proto=socket.IPPROTO_TCP
    )

    # check if it's requesting an http or https connection
    scheme, url = url.split('://', 1)

    assert scheme == 'http' or scheme == 'https'

    if ('/' in url):
        host, path = url.split('/', 1)
    else:
        host = url
        path = ""
    path = "/"+path

    port = 80

    if ":" in host:
        host, port = host.split(":", 1)
        port = int(port)
    if ":" in path:
        path, port = path.split(":", 1)
        port = int(port)

    if scheme == 'https':
        ctx = ssl.create_default_context()
        s = ctx.wrap_socket(s, server_hostname=host)
        if port == 80:
            port = 443

    # print(host,port)

    s.connect((host, port))
    s.send("GET {} HTTP/{}\r\n".format(path, http_version).encode("utf8") +
           # "Accept-Encoding: gzip\r\n".encode("utf8") +
           "Host: {}\r\n\r\n".format(host).encode("utf8"))

    # To read the response, you’d generally use the read function on sockets, which gives whatever bits of the response have already arrived.
    # Then you write a loop that collects bits of the response as they arrive.
    # However, in Python you can use the makefile helper function, which hides the loop

    response = s.makefile("r", encoding="utf8", newline="\r\n")
    # Here makefile returns a file-like object containing every byte we receive from the server

    # print(response)

    statusline = response.readline()
    # print(statusline)
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
    # print(headers)

    # Headers can describe all sorts of information,
    # but a couple of headers are especially important because they tell us that the data we’re trying to access is being sent in an unusual way.
    # Let’s make sure none of those are present

    assert "transfer-encoding" not in headers
    assert "content-encoding" not in headers

    body = response.read()
    s.close()
    # print(body)
    if ('Content-Encoding' in headers and headers['Content-Encoding'] == 'gzip'):
        body = zlib.decompress(body)  # .decode("utf8")
    return headers, body


def showBody(htmlDoc):
    bodyWithoutTags = ""
    is_tag_char = False
    #body = re.sub('<head>[\s\S]*</head>', '', htmlDoc)
    for chr in htmlDoc:
        if chr == '<':
            is_tag_char = True
        elif chr == '>':
            is_tag_char = False
        elif not is_tag_char:
            bodyWithoutTags += chr
    print(bodyWithoutTags.strip())


def load(url):
    headers, htmlDoc = request(url)
    showBody(htmlDoc)


if __name__ == "__main__":
    import sys
    load(sys.argv[1])
