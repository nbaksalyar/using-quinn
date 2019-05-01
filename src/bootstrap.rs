// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::connect;
use crate::connection::BootstrapGroupMaker;
use crate::context::ctx;

pub fn start() {
    let (proxies, event_tx): (Vec<_>, _) = ctx(|c| {
        (
            c.bootstrap_cache
                .peers()
                .iter()
                .rev()
                .chain(c.bootstrap_cache.hard_coded_contacts().iter())
                .cloned()
                .collect(),
            c.event_tx.clone(),
        )
    });

    let maker = BootstrapGroupMaker::new(event_tx);
    for proxy in proxies {
        let _ = connect::connect_to(proxy, None, Some(&maker));
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::new_random_qp2p_for_unit_test;
    use crate::{Builder, Config, Event};
    use env_logger;
    use std::collections::HashSet;
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::mpsc;

    // Try to bootstrap to 4 different peer at once. Use an artificial delay for 3 of the peers.
    // Make sure in the end we have only one bootstrapped connection (all other connections are
    // supposed to be dropped).
    #[test]
    fn bootstrap_to_only_one_node() {
        env_logger::init();

        let mut bs_nodes = Vec::new();
        let mut bs_peer_addrs = Vec::new();
        let mut hcc_contacts = HashSet::new();

        // Spin up 4 peers that we'll use for hardcoded contacts.
        for i in 0..4 {
            let (tx, rx) = mpsc::channel();
            let mut qp2p = unwrap!(Builder::new(tx)
                .with_config(Config {
                    ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                    port: Some(0),
                    ..Default::default()
                })
                .build());

            // qp2p.set_connect_delay(i * 50);

            let ci_info = unwrap!(qp2p.our_connection_info());

            bs_peer_addrs.push(ci_info.peer_addr);
            hcc_contacts.insert(ci_info);
            bs_nodes.push((rx, qp2p));
        }

        // Create a client and bootstrap to 4 of the peers we created previously.
        let (mut qp2p0, rx0) = new_random_qp2p_for_unit_test(false, hcc_contacts);
        qp2p0.bootstrap();

        match unwrap!(rx0.recv()) {
            Event::BootstrappedTo { node, .. } => {
                // assert_eq!(node.peer_addr, bs_peer_addrs[0]);
            }
            ev => panic!("Unexpected event: {:?}", ev),
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        // We expect only 1 bootstrap connection to succeed.
        let _ = unwrap!(qp2p0.connections(|c| println!("{:?}", c)));
        let conns_count = unwrap!(qp2p0.connections(|c| c.len()));
        assert_eq!(conns_count, 1);
    }
}
