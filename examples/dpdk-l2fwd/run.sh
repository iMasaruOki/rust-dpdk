#!/bin/sh
# prepare veth and namespaces
sudo ip netns add ns1
sudo ip link add dev tap-ns1 type veth peer name ns-ns1
sudo ip link set dev ns-ns1 netns ns1
sudo ip link set dev tap-ns1 up
sudo ip netns exec ns1 ip addr add 114.0.0.1/24 dev ns-ns1
sudo ip netns exec ns1 ip link set dev ns-ns1 up

sudo ip netns add ns2
sudo ip link add dev tap-ns2 type veth peer name ns-ns2
sudo ip link set dev ns-ns2 netns ns2
sudo ip link set dev tap-ns2 up
sudo ip netns exec ns2 ip addr add 114.0.0.2/24 dev ns-ns2
sudo ip netns exec ns2 ip link set dev ns-ns2 up

sudo ip netns add ns3
sudo ip link add dev tap-ns3 type veth peer name ns-ns3
sudo ip link set dev ns-ns3 netns ns3
sudo ip link set dev tap-ns3 up
sudo ip netns exec ns3 ip addr add 114.0.0.3/24 dev ns-ns3
sudo ip netns exec ns3 ip link set dev ns-ns3 up

# Run lagopus as rawsocket only mode.
sudo env RUST_BACKTRACE=1 ./target/debug/dpdk-l2fwd -cf -n2 \
 -d /usr/local/lib/dpdk-pmd \
 --vdev net_af_packet0,iface=tap-ns1 \
 --vdev net_af_packet1,iface=tap-ns2 \
 --vdev net_af_packet2,iface=tap-ns3 \
-- -p7 &
L2FWD_PID=$!

# comunication test
sudo ip netns exec ns1 ping -c 3 114.0.0.2  #1->2 OK
sudo ip netns exec ns1 ping -c 3 114.0.0.3  #1->3 OK

# cleanup
#kill -TERM $L2FWD_PID
sudo ip link del tap-ns1
sudo ip link del tap-ns2
sudo ip link del tap-ns3
sudo ip -all netns del
