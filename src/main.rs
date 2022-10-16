/*
route-guardian

Declaritively manage routes using netlink

References:
 - https://github.com/little-dude/netlink/blob/master/rtnetlink/examples/get_route.rs
 - https://github.com/little-dude/netlink/blob/master/rtnetlink/examples/add_route.rs
 - https://github.com/little-dude/netlink/blob/master/rtnetlink/src/route/del.rs
 - https://github.com/little-dude/netlink/blob/master/rtnetlink/examples/listen.rs
 - https://github.com/chyh1990/yaml-rust/blob/master/examples/dump_yaml.rs
*/

// SPDX-License-Identifier: MIT

// https://github.com/little-dude/netlink/blob/master/rtnetlink/examples/listen.rs

//! This example opens a netlink socket, registers for IPv4 and IPv6 routing changes, listens for
//! said changes and prints the received messages.

use color_eyre::Report;
use tracing::info;
use tracing::error;
use tracing_subscriber::EnvFilter;

use futures::stream::StreamExt;
use futures::stream::TryStreamExt;

use rtnetlink::{
    constants::{RTMGRP_IPV4_ROUTE, RTMGRP_IPV6_ROUTE},
    new_connection,
    sys::{AsyncSocket, SocketAddr},
    packet::rtnl::{
        link::nlas::Nla,
    },
    Handle,
    IpVersion,
};

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    // Open the netlink socket
    let (mut connection, handle, mut messages) = new_connection()?;

    // These flags specify what kinds of broadcast messages we want to listen for.
    let mgroup_flags = RTMGRP_IPV4_ROUTE | RTMGRP_IPV6_ROUTE;

    // A netlink socket address is created with said flags.
    let addr = SocketAddr::new(0, mgroup_flags);
    // Said address is bound so new conenctions and thus new message broadcasts can be received.
    connection
        .socket_mut()
        .socket_mut()
        .bind(&addr)
        .expect("failed to bind");

    tokio::spawn(connection);

    // Dump all the links and print their index and name
    info!("*** dumping links ***");
    if let Err(e) = dump_links(handle.clone()).await {
        error!("{}", e);
    }

    info!("dumping routes for IPv4");
    if let Err(e) = dump_addresses(handle.clone(), IpVersion::V4).await {
        error!("{}", e);
    }

    while let Some((message, _)) = messages.next().await {
        let payload = message.payload;
        info!("Route change message - {:?}", payload);
    }
    Ok(())
}

async fn dump_links(handle: Handle) -> Result<(), Report> {
    let mut links = handle.link().get().execute();
    'outer: while let Some(msg) = links.try_next().await? {
        for nla in msg.nlas.into_iter() {
            if let Nla::IfName(name) = nla {
                info!("found link {} ({})", msg.header.index, name);
                continue 'outer;
            }
        }
        error!("found link {}, but the link has no name", msg.header.index);
    }
    Ok(())
}

async fn dump_addresses(handle: Handle, ip_version: IpVersion) -> Result<(), Report> {
    let mut routes = handle.route().get(ip_version).execute();
    while let Some(route) = routes.try_next().await? {
        info!("{:?}", route);
    }
    Ok(())
}


fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
