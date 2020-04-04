# Virtual Interface Notes

It's possible to create a virtual interface with [vtun][vtun]. Virtual interfaces
can be created both in code and at the CLI. It _might_ be necessary to do some
combination of both, but I believe that it can be done _just_ in code as long as
the right permissions are present.

A [bridge example][bridge_ex] in c/C++ is also a really good guide and contains the
commands to run to create a tun/tap interface via the CLI.

```sh
# Enable IP forwarding
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
the virtual interface to send out legit traffic. It also may be required to use
IP-table rules to restrict all traffic to a given interface (possibly by IP assigned
to the interface).

```sh
iptables -t nat -I POSTROUTING -d DEST.IP -j SNAT --to VIRTUAL.IP
```




  [vtun]: http://vtun.sourceforge.net/tun
  [bridge_ex]: https://android.googlesource.com/platform/development/+/master/samples/ToyVpn/server/linux/ToyVpnServer.cpp
