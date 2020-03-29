/* Explicitly enable ifreq* functions.
 * This is mostly for the editor since it is implied by GCC or
 * by using a GNU specific standard, such as gnu17 */
#define _DEFAULT_SOURCE 1

#include <arpa/inet.h>       // htons
#include <errno.h>           // errno
#include <linux/if_packet.h>
#include <net/ethernet.h>    // ETH_P_* flags
#include <net/if.h>          // ifreq
#include <stdio.h>           // printf family of funcs
#include <string.h>          // memset
#include <sys/ioctl.h>       // ioctl & flags
#include <sys/socket.h>      // socket
#include <sys/types.h>       // generally useful C sys types (uid_t, pid_t, etc)
#include <unistd.h>          // close

// #include <linux/if_packet.h>

/*-----------------------------------------------------------------------------
 * Type Aliases & Utility Defines
 *-----------------------------------------------------------------------------*/

typedef int socket_fd;
typedef int error_code;

#define SUCCESS_CODE 0
#define FAILURE_CODE -1

#define min(a, b) (a < b ? a : b)


/*-----------------------------------------------------------------------------
 * Function Declarations (Pre-Defs)
 *-----------------------------------------------------------------------------*/

/**
 * @brief Initialize a socket for reading raw ethernet packets
 * 
 * @param interface name of the interface to bind to
 *        (e.g. 'eth0', 'ens1', etc.)
 * @return Socket descriptor/handle or -1 if error
 */
socket_fd init_raw(const char *interface);

void close_socket(socket_fd sock);

error_code get_interface_index(socket_fd sock,
    const char *interface,
    int *index);



/*-----------------------------------------------------------------------------
 * Main & Function Implementations
 *-----------------------------------------------------------------------------*/

int main() {
    // hard-coded to my XPS 15
    socket_fd sock = init_raw("lo");
    if (sock == FAILURE_CODE) {
        return FAILURE_CODE;
    }
    printf("Opened socket %d\n", sock);

    close_socket(sock);
    return sock;
}

socket_fd
init_raw (const char *interface) {
    /* Create a socket with the following characteristics:
     *   - PF_PACKET - Protocol Family of 'Packet'
     *   - SOCK_RAW  - Raw bytes (get entire protocol packet in data)
     *   - ETH_P_ALL - Filter for all types of ethernet packets
     * 
     * TODO: play around with DIX vs 802.3 in type to filter by and
     *       find a filter that works for our experiments.
     */
    socket_fd sock = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
    if (sock < 0) {
        printf("Failed to open socket. Errno: %d\n", errno);
        return FAILURE_CODE;
    }
    
    // Get the index of the interface (from parameter)
    int interface_index;
    if (get_interface_index(sock, interface, &interface_index) == FAILURE_CODE) {
        // signal failure by returning -1
        close_socket(sock);
        return -1;
    }
    printf("Interface value: %u\n", interface_index);

    // use the interface index to 
    struct sockaddr_ll addr;
    addr.sll_family = AF_PACKET;
    addr.sll_protocol = htons(ETH_P_ALL);
    addr.sll_ifindex = interface_index;

    int err = bind(sock, (struct sockaddr *) &addr, sizeof(struct sockaddr_ll));
    if (err < 0) {
        printf("couldn't bind. return code: %d, errno: %d\n", err, errno);
        return -1;
    }
    printf("bound. return code: %d\n", err);

    return sock;
}


error_code
get_interface_index(socket_fd sock, const char *interface, int *index) {

    // Create an interface request object to use to query via ioctl
    struct ifreq ifr;
    printf("sizeof ifr.ifr_ifru: %lu\n", sizeof(ifr.ifr_ifru));
    memset(&ifr, 0, sizeof(ifr));
    strncpy(ifr.ifr_name, interface,
        min(sizeof(ifr.ifr_name), strlen(interface) * sizeof(char)));

    // Get the index of the interface name (given in parameters)
    if (ioctl(sock, SIOCGIFINDEX, &ifr) < 0 || ifr.ifr_ifindex < 0) {
        return FAILURE_CODE;
    }

    *index = ifr.ifr_ifindex;

    return SUCCESS_CODE;
}

void
close_socket(socket_fd sock) {
    if (sock < 0) return;

    if (close(sock) < 0) {
        printf("Error encountered closing socket: %d", errno);
    }
}
