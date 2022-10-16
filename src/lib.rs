use rtnetlink::{
    constants::{RTMGRP_IPV4_ROUTE, RTMGRP_IPV6_ROUTE},
    new_connection,
    sys::{AsyncSocket, SocketAddr},
    packet::{rtnl::{
        link::nlas::Nla,
    }, RtnlMessage},
    Handle,
    IpVersion,
};

use std::net::{
    Ipv4Addr,
    Ipv6Addr,
};

use cidr_utils::cidr::IpCidr;

use netlink_proto::Connection;

enum RouteManagerMode
{
    Normal,
    Strict,
    Dictator,
}

struct IPv4Route
{
    dest: IpCidr,
    gateway: Ipv4Addr,
    device: String,
}

// TODO: Implement IPv6
// struct IPv6Route
// {

// }

struct RoutingTable
{
    name: String,
    index: u8,
    ipv4_routes: Vec<IPv4Route>,
    // ipv6_routes: Vec(IPv6Route),
}

struct route_manager
{
    routing_tables: Vec<RoutingTable>,
    mode: RouteManagerMode,
    netlink_connection: Connection<RtnlMessage>,
    netlink_handle: Handle,
}