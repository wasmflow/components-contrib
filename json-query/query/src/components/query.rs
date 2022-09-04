use jaq_core::{parse, Ctx, Definitions, Val};
use markup_converter::Format;
use serde_json::Value;
use std::error;

pub use crate::components::generated::query::*;

#[async_trait::async_trait]

impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        _config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filter = &inputs.filter;

        // parse the filter in the context of the given definitions
        let mut errs = Vec::new();
        let (filters, errors) = parse::parse(filter, parse::main());

        let data: Value = serde_json::from_str(&inputs.input)?;

        if !errors.is_empty() {
            return Err("Errors parsing queries".into());
        }
        if let Some(filters) = filters {
            // start out only from core filters,
            // which do not include filters in the standard library
            // such as `map`, `select` etc.
            let defs = Definitions::core();

            let filters = defs.finish(filters, Vec::new(), &mut errs);

            // iterator over the output values
            let out = filters.run(Ctx::new(), Val::from(data));
            let mut results: Vec<String> = Vec::new();

            for val in out {
                match val {
                    Ok(result) => {
                        results.push(result.to_string());
                    }
                    Err(e) => return Err(format!("Error: {}", e).into()),
                };
            }
            outputs.output.done(results)?;
        } else {
            // debug!("No queries successfully parsed");
        }

        Ok(())
    }
}
