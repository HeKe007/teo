use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::database::r#type::DatabaseType;
use crate::core::env::Env;
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::error::ActionError;
use crate::core::result::ActionResult;
use crate::prelude::Value;

#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn save_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> ActionResult<()>;

    async fn delete_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> ActionResult<()>;

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, env: Env) -> Result<Object, ActionError>;

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, env: Env) -> Result<Vec<Object>, ActionError>;

    async fn count(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<usize, ActionError>;

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value, ActionError>;

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value, ActionError>;

    fn new_save_session(&self) -> Arc<dyn SaveSession>;
}

#[async_trait]
pub(crate) trait ConnectorBuilder: Debug + Send + Sync {

    fn default_database_type(&self, field_type: &FieldType) -> DatabaseType;

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector>;
}

#[async_trait]
pub(crate) trait SaveSession: Debug + Send + Sync { }
