use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::save_session::SaveSession;
use crate::core::error::ActionError;
use crate::core::result::ActionResult;

#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn save_object(&self, object: &Object) -> ActionResult<()>;

    async fn delete_object(&self, object: &Object) -> ActionResult<()>;

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError>;

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Vec<Object>, ActionError>;

    async fn count(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<usize, ActionError>;

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<JsonValue, ActionError>;

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<JsonValue, ActionError>;

    fn new_save_session(&self) -> Arc<dyn SaveSession>;
}

#[async_trait]
pub(crate) trait ConnectorBuilder: Debug + Send + Sync {

    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType;

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector>;
}
