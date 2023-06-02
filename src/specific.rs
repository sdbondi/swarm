use std::future::pending;
use std::ops::RangeInclusive;
use std::path::Path;
use tari_common_types::types::PublicKey;
use tari_utilities::hex::Hex;
use tari_validator_node_client::types::AddPeerRequest;
use tokio::{process, time};

pub async fn do_specific_things(node_range: RangeInclusive<usize>) -> anyhow::Result<()> {
    let mut validators = vec![];
    for i in node_range {
        validators.push(ValidatorNode::spawn(
            20000 + i as u16,
            19000 + i as u16,
            format!("/home/stan/Litterbox/validators/vn{i}"),
        )?);

        println!("Spawned validator node {}", i);
        time::sleep(time::Duration::from_millis(100)).await;
    }

    time::sleep(time::Duration::from_secs(5)).await;

    for validator in &validators {
        println!("Adding validator node {}", validator.jrpc_port);
        validator.add_peer().await?;
        // println!("Registering validator node {}", validator.jrpc_port);
        // validator.register().await?;
    }

    // Wait forever
    pending::<()>().await;

    Ok(())
}

pub struct ValidatorNode {
    pub port: u16,
    pub jrpc_port: u16,
    pub process: process::Child,
}

impl ValidatorNode {
    pub fn spawn<P: AsRef<Path>>(port: u16, jrpc_port: u16, base_dir: P) -> anyhow::Result<Self> {
        let child = process::Command::new("/home/stan/tari/dan/target/debug/tari_validator_node")
            .args(&[
                "--base-path",
                base_dir.as_ref().to_str().unwrap(),
                "--rpc-address",
                &format!("127.0.0.1:{jrpc_port}"),
                "--network",
                "localnet",
                "-plocalnet.p2p.seeds.peer_seeds=52bef4d946a13d3cc39f3adc11471b551109e0f3f7a726a941b4a7d3c3896160::/ip4/127.0.0.1/tcp/12345",
                "-pvalidator_node.p2p.transport.type=tcp",
                &format!("-pvalidator_node.p2p.transport.tcp.listener_address=/ip4/127.0.0.1/tcp/{port}"),
                &format!("-pvalidator_node.public_address=/ip4/127.0.0.1/tcp/{port}"),
                "-pvalidator_node.p2p.allow_test_addresses=true",
            ])
            .kill_on_drop(true)
            .spawn()?;

        Ok(Self {
            port,
            jrpc_port,
            process: child,
        })
    }

    pub async fn add_peer(&self) -> anyhow::Result<()> {
        let mut client = tari_validator_node_client::ValidatorNodeClient::connect(format!(
            "http://127.0.0.1:{}",
            self.jrpc_port
        ))?;
        client
            .add_peer(AddPeerRequest {
                public_key: PublicKey::from_hex(
                    "52bef4d946a13d3cc39f3adc11471b551109e0f3f7a726a941b4a7d3c3896160",
                )?,
                addresses: vec!["/ip4/127.0.0.1/tcp/12345".parse()?],
                wait_for_dial: false,
            })
            .await?;
        Ok(())
    }

    pub async fn register(&self) -> anyhow::Result<()> {
        let mut client = tari_validator_node_client::ValidatorNodeClient::connect(format!(
            "http://127.0.0.1:{}",
            self.jrpc_port
        ))?;
        client.register_validator_node().await?;
        Ok(())
    }
}
