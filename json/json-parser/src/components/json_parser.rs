pub use crate::components::generated::json_parser::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        _config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let myvalue: Result<serde_json::Value, _> = serde_json::from_str(&inputs.input);
        outputs.output.done(myvalue?.to_string())?;
        Ok(())
    }
}
