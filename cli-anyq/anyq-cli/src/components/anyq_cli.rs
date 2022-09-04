pub use crate::components::generated::anyq_cli::*;

use clap;
use clap::Parser;
use std::io;
use std::io::{BufRead, BufReader};
use wasmflow_sdk::v1::packet::PacketMap;

#[derive(Debug, Clone, Parser)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct Options {
    /// The query to filter the input with.
    #[clap(action)]
    filter: String,

    #[clap(action, long = "type", short = 't', default_value = "json")]
    kind: String,
}

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let opts = Options::parse_from(inputs.argv);
        let reader = BufReader::new(io::stdin());

        let input = reader
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .join("\n");

        let payload = PacketMap::from([("input", input), ("filter", opts.filter)]);

        let mut stream = inputs.program.unwrap().call(&opts.kind, payload).await?;
        let packets = stream.drain_port("output").await?;
        for packet in packets {
            let results: Vec<String> = packet.deserialize()?;
            for result in results {
                println!("{}", result);
            }
        }

        Ok(())
    }
}