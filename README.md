# MPNC: Multipath single-Player NetCat

TL;DR: A PoC to make your link aggregation works.

## Background

We all know NC is useful in real world:

```bash
# on server A
cat /dev/disk1s1 | nc newServer 31337
# on server B
nc -l 31337 | pv >/dev/disk1s1
```

However, for poor guys like me, we only have some gigabit network cards. Due to `nc` is only using one TCP connection, the maximum speed is 1Gbps.

Luckily, in 2000, [Linux Ethernet Bonding](https://www.kernel.org/doc/Documentation/networking/bonding.txt) is introduced. This allows us using multiple low
speed NIC cards to get a high speed connection `bond0`. This feature is pretty useful if what you want is a router, firewall, or some other devices that mainly
perform network jobs.

But ethernet bonding has its own limitations: it's still a bonding, not some "smart" load balancing. Bonding works for router because it allows you to use
different interface to communicate with different peers (another server, or router), or have an active-backup mechanism. But when only one TCP connections
wants to get speed more than one physical interface, bonding is not helpful.

## Problem

So the problem is simple: how to make the speed of single TCP connection can go beyond the speed of the physical devices?

## Possible solutions

There do have some possible solutions. One of the most useful option is [Multipath TCP](https://www.multipath-tcp.org/), which I had some experience in
production environment. But this time MPTCP is not useful for me, because according to its introduction:

> ... enabling the simultaneous use of several IP-addresses/interfaces ...

And we only have one bonding interface, and one IP address for the both peer. It's, surely, possible to setup additional address, or just deactivate the
bonding, but I do not want to touch living network devices during Spring Festival to increase risks of being accused by users. Also, besides the kernel support
for MPTCP might not be available for my server, I still need to use some "hack" methods like [systemtap](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/configuring_and_managing_networking/getting-started-with-multipath-tcp_configuring-and-managing-networking)
or `LD_PRELOAD`. Neither of them should be used unless no other options is available.

## This solution

This is a brute solution. If one TCP connection speed is limited, add more.

The bonding hash policy, or [`xmit_hash_policy`](https://www.kernel.org/doc/Documentation/networking/bonding.txt#:~:text=value%20is%201.-,xmit_hash_policy,-Selects%20the%20transmit),
decides which interfaces should be used to send a packet in 802.3ad mode (which is used in our system). If we use `layer3+4`, then we can let the traffic
goes to different physical interface, by setting the traffic's source and destination port, leaving IP/MAC addresses untouched.

So bad news is: this solution is bruteforce but quick, and it works like netcat - but only in one way (client -> server).

Good news is: it works! In our datacenter where two servers are connected via 4 Gigabit ethernet card through a switch, the transmit speed is around 400MiB/s!
This is enough to run migration scripts!

## Should I use it in production?

Not now. This is just a PoC.

Tell me if you *really* need more features, like setting ports/threads automatically based on bonding configuration, or using only one port at server side
to minimize firewall settings, or make it looks like a real NetCat by adding additional properties and add duplex abilities.

But I would recommend you just go out and by a 10Gbps NIC. It's not that cheap and more stable.

## License

GPLv2
