// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod common;
use common::*;

use snarkos_node_tcp::{protocols::Handshake, P2P};

use core::time::Duration;

#[tokio::test]
async fn test_connect_without_handshake() {
    // Create 2 routers.
    let node0 = validator(0, 2).await;
    let node1 = client(0, 2).await;
    assert_eq!(node0.number_of_connected_peers(), 0);
    assert_eq!(node1.number_of_connected_peers(), 0);

    // Start listening.
    node0.tcp().enable_listener().await.unwrap();
    node1.tcp().enable_listener().await.unwrap();

    {
        // Connect node0 to node1.
        node0.connect(node1.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        assert_eq!(node0.tcp().num_connected(), 1);
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 1);
        assert_eq!(node1.tcp().num_connecting(), 0);
    }
    {
        // Connect node0 from node1 again.
        node0.connect(node1.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        assert_eq!(node0.tcp().num_connected(), 1);
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 1);
        assert_eq!(node1.tcp().num_connecting(), 0);
    }
    {
        // Connect node1 from node0.
        node1.connect(node0.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        assert_eq!(node0.tcp().num_connected(), 2); // node0 has no way of deduping the connection.
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 2); // node1 has no way of deduping the connection.
        assert_eq!(node1.tcp().num_connecting(), 0);
    }
}

#[tokio::test]
async fn test_connect_with_handshake() {
    // Create 2 routers.
    let node0 = validator(0, 2).await;
    let node1 = client(0, 2).await;
    assert_eq!(node0.number_of_connected_peers(), 0);
    assert_eq!(node1.number_of_connected_peers(), 0);

    // Enable handshake protocol.
    node0.enable_handshake().await;
    node1.enable_handshake().await;

    // Start listening.
    node0.tcp().enable_listener().await.unwrap();
    node1.tcp().enable_listener().await.unwrap();

    {
        // Connect node0 to node1.
        node0.connect(node1.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        // Check the TCP level.
        assert_eq!(node0.tcp().num_connected(), 1);
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 1);
        assert_eq!(node1.tcp().num_connecting(), 0);

        // Check the router level.
        assert_eq!(node0.number_of_connected_peers(), 1);
        assert_eq!(node1.number_of_connected_peers(), 1);
    }
    {
        // Connect node0 to node1 again.
        node0.connect(node1.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        // Check the TCP level.
        assert_eq!(node0.tcp().num_connected(), 1);
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 1);
        assert_eq!(node1.tcp().num_connecting(), 0);

        // Check the router level.
        assert_eq!(node0.number_of_connected_peers(), 1);
        assert_eq!(node1.number_of_connected_peers(), 1);
    }
    {
        // Connect node1 to node0.
        node1.connect(node0.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        // Check the TCP level.
        assert_eq!(node0.tcp().num_connected(), 1);
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 1);
        assert_eq!(node1.tcp().num_connecting(), 0);

        // Check the router level.
        assert_eq!(node0.number_of_connected_peers(), 1);
        assert_eq!(node1.number_of_connected_peers(), 1);
    }
}

#[ignore]
#[tokio::test]
async fn test_connect_simultaneously_with_handshake() {
    // Create 2 routers.
    let node0 = validator(0, 2).await;
    let node1 = client(0, 2).await;
    assert_eq!(node0.number_of_connected_peers(), 0);
    assert_eq!(node1.number_of_connected_peers(), 0);

    // Enable handshake protocol.
    node0.enable_handshake().await;
    node1.enable_handshake().await;

    {
        // Connect node0 to node1.
        node0.connect(node1.local_ip());
        // Connect node1 to node0.
        node1.connect(node0.local_ip());
        // Sleep briefly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        print_tcp!(node0);
        print_tcp!(node1);

        // Check the TCP level.
        assert_eq!(node0.tcp().num_connected(), 1);
        assert_eq!(node0.tcp().num_connecting(), 0);
        assert_eq!(node1.tcp().num_connected(), 1);
        assert_eq!(node1.tcp().num_connecting(), 0);

        // Check the router level.
        assert_eq!(node0.number_of_connected_peers(), 1);
        assert_eq!(node1.number_of_connected_peers(), 1);
    }
}
