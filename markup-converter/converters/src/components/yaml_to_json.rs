pub use crate::components::generated::yaml_to_json::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let myvalue: Result<serde_json::Value, _> = serde_yaml::from_str(&inputs.input);
        outputs.output.done(myvalue?.to_string())?;
        Ok(())
    }
}
