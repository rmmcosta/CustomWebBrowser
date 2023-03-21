// socket-client project main.go
package make_request

import (
	"crypto/tls"
	"fmt"
	"io/ioutil"
	"net"
	"net/http"
	"strconv"
	"strings"
)

const (
	SERVER_TYPE    = "tcp"
	HTTP_VERSION   = "1.0"
	BUFFER_SIZE    = 40960
	REQUEST_METHOD = "GET"
)

func MakeRequest(url string) string {
	// parse url
	if !strings.Contains(url, "://") {
		panic("Invalide Url!")
	}

	//parse domain and path to extract the port
	port, host := getPort(url)

	//establish connection
	host, path := getPathAndHost(host)
	path = "/" + path
	hostAndPort := host + ":" + strconv.Itoa(port)
	//fmt.Println(hostAndPort)
	request := buildRequest(host, path, HTTP_VERSION)
	var response string
	if port == 443 {
		response = httpsRequestWithRawSocket(request, SERVER_TYPE, hostAndPort, host)
	} else {
		response = httpRequest(request, SERVER_TYPE, hostAndPort)
	}
	return response
}

func httpRequest(request string, serverType string, hostAndPort string) string {
	connection, err := net.Dial(serverType, hostAndPort)
	if err != nil {
		panic(err)
	}
	_, err = connection.Write([]byte(request))
	buffer := make([]byte, BUFFER_SIZE)
	mLen, err := connection.Read(buffer)
	if err != nil {
		panic(err)
	}
	defer connection.Close()
	return string(buffer[:mLen])
}

// only returns the headers, not enough for what we need
func httpsRequest(request string, serverType string, hostAndPort string, host string) string {
	tlsConfig := &tls.Config{
		ServerName: host,
		//InsecureSkipVerify: true,
	}
	connection, err := tls.Dial(serverType, hostAndPort, tlsConfig)
	if err != nil {
		panic(err)
	}
	_, err = connection.Write([]byte(request))
	buffer := make([]byte, BUFFER_SIZE)
	mLen, err := connection.Read(buffer)
	if err != nil {
		panic(err)
	}
	defer connection.Close()
	return string(buffer[:mLen])
}

func httpsRequestWithRawSocket(request string, serverType string, hostAndPort string, host string) string {
	// Connect to the remote server using TCP
	conn, err := net.Dial(serverType, hostAndPort)
	if err != nil {
		panic(err)
	}

	// Create a new TLS client with the default configuration
	tlsConn := tls.Client(conn, &tls.Config{
		ServerName: host,
	})

	// Handshake with the remote server
	if err := tlsConn.Handshake(); err != nil {
		panic(err)
	}

	// Send an HTTP request to the server
	if _, err := tlsConn.Write([]byte(request)); err != nil {
		panic(err)
	}

	// Read the server's response headers
	var responseHeaders []byte
	buffer := make([]byte, 1024)
	for {
		n, err := tlsConn.Read(buffer)
		if err != nil {
			panic(err)
		}
		responseHeaders = append(responseHeaders, buffer[:n]...)
		if n < len(buffer) || strings.Contains(string(responseHeaders), "\r\n\r\n") {
			break
		}
	}

	// Extract the length of the message body from the response headers
	contentLength := -1
	for _, headerLine := range strings.Split(string(responseHeaders), "\r\n") {
		headerFields := strings.SplitN(headerLine, ":", 2)
		if len(headerFields) == 2 && strings.TrimSpace(headerFields[0]) == "Content-Length" {
			fmt.Sscan(strings.TrimSpace(headerFields[1]), &contentLength)
			break
		}
	}

	// Read the server's response body
	responseBody := make([]byte, contentLength)
	if contentLength > 0 {
		if _, err := tlsConn.Read(responseBody); err != nil {
			panic(err)
		}
	}

	// Print the complete response
	completeResponse := append(responseHeaders, responseBody...)

	// Close the TLS connection
	defer tlsConn.Close()
	return string(completeResponse)
}

func httpsRequestWithHttpRequest(request string, serverType string, url string, host string) string {
	req, err := http.NewRequest(REQUEST_METHOD, url, nil)
	if err != nil {
		panic(err)
	}

	// Send the request using an HTTP client with a custom Transport
	transport := &http.Transport{
		TLSClientConfig: &tls.Config{
			ServerName: host,
		},
	}
	client := &http.Client{Transport: transport}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()

	// Read the HTML content from the response body using ioutil.ReadAll
	htmlBytes, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	response := make([]byte, BUFFER_SIZE)
	for key, values := range resp.Header {
		response = append(response, []byte(key+":"+values[0]+"\n")...)
	}
	return string(append(response, htmlBytes...))
}

func buildRequest(host string, path string, http_version string) string {
	request := fmt.Sprintf("GET %s HTTP/%s\r\nHost: %s\r\n\r\n", path, HTTP_VERSION, host)
	//fmt.Println(request)
	return request
}

func getPort(url string) (int, string) {
	urlSliceScheme := strings.Split(url, "://")
	scheme := urlSliceScheme[0]
	domainAndPath := urlSliceScheme[1]

	if strings.Contains(domainAndPath, ":") {
		portSlice := strings.Split(domainAndPath, ":")
		port, err := strconv.Atoi(portSlice[1])
		if err != nil {
			panic("Invalid Port!")
		}
		return port, portSlice[0]
	}

	if scheme == "https" {
		return 443, domainAndPath
	}

	return 80, domainAndPath
}

func getPathAndHost(host string) (string, string) {
	if strings.Contains(host, "/") {
		return strings.Split(host, "/")[0], strings.Split(host, "/")[1]
	}
	return host, ""
}
