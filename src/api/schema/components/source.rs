use super::{sink, state, transform, Component};
use crate::{api::schema::metrics, config::DataType};
use async_graphql::{Enum, Object};

#[derive(Debug, Enum, Eq, PartialEq, Copy, Clone)]
pub enum SourceType {
    File,
}

#[derive(Debug, Enum, Eq, PartialEq, Copy, Clone)]
pub enum SourceOutputType {
    Any,
    Log,
    Metric,
}

impl From<DataType> for SourceOutputType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Metric => SourceOutputType::Metric,
            DataType::Log => SourceOutputType::Log,
            DataType::Any => SourceOutputType::Any,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
    pub component_type: String,
    pub output_type: DataType,
}

#[derive(Debug, Clone)]
pub struct Source(pub Data);

#[Object]
impl Source {
    /// Source name
    pub async fn name(&self) -> &str {
        &*self.0.name
    }

    /// Source type
    pub async fn component_type(&self) -> &str {
        &*self.0.component_type
    }

    /// Source output type
    pub async fn output_type(&self) -> SourceOutputType {
        self.0.output_type.into()
    }

    /// Transform outputs
    pub async fn transforms(&self) -> Vec<transform::Transform> {
        state::filter_components(|(_name, components)| match components {
            Component::Transform(t) if t.0.inputs.contains(&self.0.name) => Some(t.clone()),
            _ => None,
        })
    }

    /// Sink outputs
    pub async fn sinks(&self) -> Vec<sink::Sink> {
        state::filter_components(|(_name, components)| match components {
            Component::Sink(s) if s.0.inputs.contains(&self.0.name) => Some(s.clone()),
            _ => None,
        })
    }

    /// Metric indicating events processed for the current source
    pub async fn processed_events_total(&self) -> Option<metrics::ProcessedEventsTotal> {
        metrics::component_processed_events_total(&self.0.name)
    }

    /// Metric indicating bytes processed for the current source
    pub async fn processed_bytes_total(&self) -> Option<metrics::ProcessedBytesTotal> {
        metrics::component_processed_bytes_total(&self.0.name)
    }
}