# Virtual Interface Notes

It's possible to create a virtual interface with [vtun][vtun]. Virtual interfaces
can be created both in code and at the CLI. It _might_ be necessary to do some
combination of both, but I believe that it can be done _just_ in code as long as
the right permissions are present.

A [bridge example][bridge_ex] in c/C++ is also a really good guide and contains the
commands to run to create a tun/tap interface via the CLI.

```sh
# Enable IP forwarding
#   What is IP Forwarding and why is it needed?
#   https://unix.stackexchange.com/questions/14056/what-is-kernel-ip-forwarding
echo 1 > /proc/sys/net/ipv4/ip_forward

# Pick a range of private addresses and perform NAT over eth0.
iptables -t nat -A POSTROUTING -s 10.0.0.0/8 -o eth0 -j MASQUERADE

# Create a TUN interface.
ip tuntap add dev tun0 mode tun

# Set the addresses and bring up the interface.
ifconfig tun0 10.0.0.1 dstaddr 10.0.0.2
```

# Bridging

A network bridge may just be a by-product of tun/tap, but it will be necessary for
the virtual interface to send out legit traffic. As far as I can tell, a bridge is
in-kernel (software) switch that operates much like a physical one. It also may be
required to use IP-table rules to restrict all traffic to a given interface
(possibly by IP assigned to the interface).

```sh
# An IP-tables rule that forwards all trafic to an IP address that has been assigned
# to a virtual interface. I'm not sure if this is necessary in the advent of bridging
# or not.
iptables -t nat -I POSTROUTING -d DEST.IP -j SNAT --to VIRTUAL.IP
```

# Linking TCP/UDP sockets to interfaces

TCP/UDP sockets tend to bind on a particular IP address that either represents a
single IP address or "all addresses" (e.g. 0.0.0.0). The virt-intf will need to
handle both of these use-cases, so it's safest to assume that any locally running
applications will bind to "all addresses".

This means that we will likely need to create some IP rules to enforce routing.
Something such as:

  + Setup a sub-net for all nodes in the cluster under `192.168.0.0/16`
  + Setup all virt-intfs in the cluster under `10.0.0.0/16` and each
    node under `10.0.X.0/8`
  + Setup an IP rule that any incoming traffic to a `10.0.0.0/16` flow
    to the virt-intf
  + Setup an IP rule that any outgoing traffic to a `10.0.0.0/16` flow
    to the virt-intf

In order to bind to a specific interface the programmer must both go out of their
way to do so and also be running with the `cap_net_raw` capability. We will have
to draw the line here in terms of support (unless it is configurable to read from
a virt-inf).

Reading resources:
  + [How to force TCP packets to use a specific NIC][so_bind_1]
  + [bind socket to network interface][so_bind_2]




  [vtun]: http://vtun.sourceforge.net/tun
  [bridge_ex]: https://android.googlesource.com/platform/development/+/master/samples/ToyVpn/server/linux/ToyVpnServer.cpp
  [so_bind_1]: https://unix.stackexchange.com/questions/238575/how-to-force-tcp-packets-to-use-a-specific-nic
  [so_bind_2]: https://stackoverflow.com/questions/14478167/bind-socket-to-network-interface
