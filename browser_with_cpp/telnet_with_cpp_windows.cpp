#include <stdio.h>
#include <string.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include "StringBuilder.hpp"
#include <iostream>
using namespace std;

#pragma comment(lib, "ws2_32.lib") // Link with ws2_32.lib library

#define PORT 80
#define MAX_BUF_SIZE 1024 * 1024 // 1MB

const char *getIp(char *host, ADDRESS_FAMILY ai_family)
{
    struct addrinfo hints, *res;
    int status;
    char *ipstr = NULL;

    memset(&hints, 0, sizeof(hints));
    hints.ai_family = ai_family; // Use AF_INET6 to force IPv6, AF_INET for IPv4
    hints.ai_socktype = SOCK_STREAM;

    if ((status = getaddrinfo(host, NULL, &hints, &res)) != 0)
    {
        fprintf(stderr, "getaddrinfo error: %s\n", gai_strerror(status));
        return NULL;
    }

    void *addr;
    char *ipver;

    // Get the pointer to the address itself, different fields in IPv4 and IPv6
    if (res->ai_family == AF_INET)
    { // IPv4
        struct sockaddr_in *ipv4 = (struct sockaddr_in *)res->ai_addr;
        addr = &(ipv4->sin_addr);
        ipver = "IPv4";
    }
    else
    { // IPv6
        struct sockaddr_in6 *ipv6 = (struct sockaddr_in6 *)res->ai_addr;
        addr = &(ipv6->sin6_addr);
        ipver = "IPv6";
    }

    // Convert the IP to a string and print it
    ipstr = (char *)malloc(INET6_ADDRSTRLEN);
    if (ipstr == NULL)
    {
        fprintf(stderr, "malloc error\n");
        freeaddrinfo(res);
        return NULL;
    }
    inet_ntop(res->ai_family, addr, ipstr, INET6_ADDRSTRLEN);
    printf("%s: %s\n", ipver, ipstr);

    freeaddrinfo(res); // Free the linked list

    return ipstr;
}

int main(int argc, char const *argv[])
{
    WSADATA wsa;
    SOCKET client_fd;
    struct sockaddr_in serv_addr;
    char *getHtml = "GET / HTTP/1.0\r\nHost: example.org\r\n\r\n";

    // Initialize Winsock
    if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0)
    {
        printf("WSAStartup failed: %d\n", WSAGetLastError());
        return 1;
    }

    if ((client_fd = socket(AF_INET, SOCK_STREAM, 0)) == INVALID_SOCKET)
    {
        printf("Socket creation error: %d\n", WSAGetLastError());
        return 1;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(PORT);

    // Convert IPv4 and IPv6 addresses from text to binary
    // form
    const char *ip = getIp("example.com", AF_INET);
    printf("ip: %s\n", ip);
    printf("ip: %s\n", "93.184.216.34");
    serv_addr.sin_addr.s_addr = inet_addr(ip);
    if (serv_addr.sin_addr.s_addr <= 0)
    {
        printf(
            "Invalid address/ Address not supported \n");
        return 1;
    }

    if (connect(client_fd, (struct sockaddr *)&serv_addr,
                sizeof(serv_addr)) == SOCKET_ERROR)
    {
        printf("Connection Failed: %d\n", WSAGetLastError());
        return 1;
    }

    send(client_fd, getHtml, strlen(getHtml), 0);
    printf("Request sent\n");

    // you need to continue reading the response from the server until all the data has been received.
    // The recv() function can return fewer bytes than the size of the buffer, so you need to call it repeatedly until you have received all the data.
    //  Read the response
    int valread;
    char buffer[1024] = {0};
    StringBuilder sb;
    while ((valread = recv(client_fd, buffer, sizeof(buffer), 0)) > 0)
    {
        printf("valread=%d\n", valread);
        if (valread == 0)
        {
            // Buffer overflow detected
            std::cout << "buffer overflow" << std::endl;
            break;
        }
        if (valread == SOCKET_ERROR)
        {
            printf("Error receiving data: %d\n", WSAGetLastError());
            break;
        }
        sb.append(buffer);
        // Clear the buffer to all null characters
        memset(buffer, 0, sizeof(buffer));
    }

    std::cout << sb.str() << std::endl;

    // closing the connected socket
    closesocket(client_fd);

    WSACleanup(); // Cleanup Winsock
    return 0;
}
