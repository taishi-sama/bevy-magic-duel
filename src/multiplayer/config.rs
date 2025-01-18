use std::{net::{Ipv4Addr, SocketAddr, SocketAddrV4}, time::Duration};

use client::{Authentication, ClientConfig, NetcodeConfig};
use lightyear::prelude::*;
use server::ServerConfig;


pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        // How often the client will send packets to the server (by default it is every frame).
        // Currently, the client only works if it sends packets every frame, for proper input handling.
        //c: Duration::default(),
        // How often the server will send packets to clients? You can reduce this to save bandwidth.
        server_replication_send_interval: Duration::from_millis(0),
        // The tick rate that will be used for the FixedUpdate schedule
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        // Here we make the `Mode` an argument so that we can run `lightyear` either in `Separate` mode (distinct client and server apps)
        // or in `HostServer` mode (the server also acts as a client).
        mode,
    }
}

pub fn client_config() -> ClientConfig {
    ClientConfig { 
        shared: shared_config(Mode::Separate), 
        net: client::NetConfig::Netcode { auth: Authentication::Manual {
            server_addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080).into(),
            client_id: 2,
            private_key: [0; 32],
            protocol_id: 1,
        }, 
        config: client::NetcodeConfig::default(), 
        io:  client::IoConfig::from_transport(client::ClientTransport::UdpSocket(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)))  },
        ..Default::default()
    }
}
pub fn server_config() -> ServerConfig {
    ServerConfig {
        shared: shared_config(Mode::Separate),
        net: vec![server::NetConfig::Netcode { config: server::NetcodeConfig::default().with_key([0;32]).with_protocol_id(1), 
        io: server::IoConfig::from_transport(server::ServerTransport::UdpSocket(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080))), }],
        ..Default::default()
    }
}