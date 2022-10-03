use mongodb::options::ClientOptions;
use serde_json::{json};
use tokio::test;
use teo::core::graph::Graph;
use teo::core::value::Value;
use teo::core::error::ActionError;


async fn make_graph() -> &Graph {

    let graph = Box::leak(Box::new(Graph::new(|g| {

        g.data_source().mongodb("mongodb://localhost:27017/teotestproperties");

        g.reset_database();

        g.model("Required", |m| {
            m.field("string", |f| {
                f.required().string();
            });
        });

        g.model("Optional", |m| {
            m.field("string", |f| {
                f.optional().string();
            });
        });

        g.model("Readonly", |m| {
            m.field("readonly", |f| {
                f.readonly().optional().string();
            });
        });

        g.model("Writeonly", |m| {
            m.field("writeonly", |f| {
                f.writeonly().optional().string();
            });
        });

        g.model("Internal", |m| {
            m.field("internal", |f| {
                f.internal().optional().string();
            });
        });
    }).await));

    graph
}

#[test]
async fn optional_field_if_no_input_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.create_object("Optional", json!({})).unwrap();
    let _ = simple.set_json(&json!({})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn optional_field_if_input_is_null_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.create_object("Optional", json!({})).unwrap();
    let _ = simple.set_json(&json!({"string": null})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn required_field_if_no_input_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.create_object("Required", json!({})).unwrap();
    let _ = simple.set_json(&json!({})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn required_field_if_input_is_null_returns_none() {
    let graph = make_graph().await;
    let simple = graph.create_object("Required", json!({})).unwrap();
    let _ = simple.set_json(&json!({"string": null})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn readonly_field_cannot_accept_value_through_set_json() {
    let graph = make_graph().await;
    let simple = graph.create_object("Readonly", json!({})).unwrap();
    let result = simple.set_json(&json!({"readonly": "my_value"})).await;
    assert_eq!(result.err().unwrap(), ActionError::keys_unallowed());
}

#[test]
async fn readonly_field_can_accept_value_through_set_value() {
    let graph = make_graph().await;
    let simple = graph.create_object("Readonly", json!({})).unwrap();
    let _ = simple.set_value("readonly", Value::String("ok".to_string()));
    let value = simple.get_value("readonly");
    assert_eq!(value.unwrap().unwrap(), Value::String("ok".to_string()));
}

#[test]
async fn writeonly_field_cannot_output_into_to_json() {
    let graph = make_graph().await;
    let simple = graph.create_object("Writeonly", json!({})).unwrap();
    let _ = simple.set_json(&json!({"writeonly": "123"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("writeonly"), None);
}

#[test]
async fn writeonly_field_value_can_be_get_through_get_value() {
    let graph = make_graph().await;
    let simple = graph.create_object("Writeonly", json!({})).unwrap();
    let _ = simple.set_json(&json!({"writeonly": "123"})).await;
    let value = simple.get_value("writeonly").unwrap().unwrap();
    assert_eq!(value, Value::String("123".to_string()));
}

#[test]
async fn internal_field_cannot_accept_value_through_set_json() {
    let graph = make_graph().await;
    let simple = graph.create_object("Internal", json!({})).unwrap();
    let result = simple.set_json(&json!({"internal": "my_value"})).await;
    assert_eq!(result.err().unwrap(), ActionError::keys_unallowed());
}

#[test]
async fn internal_field_can_accept_value_through_set_value() {
    let graph = make_graph().await;
    let simple = graph.create_object("Internal", json!({})).unwrap();
    let _ = simple.set_value("internal", Value::String("ok".to_string()));
    let value = simple.get_value("internal");
    assert_eq!(value.unwrap().unwrap(), Value::String("ok".to_string()));
}

#[test]
async fn internal_field_cannot_output_into_to_json() {
    let graph = make_graph().await;
    let simple = graph.create_object("Internal", json!({})).unwrap();
    let _ = simple.set_json(&json!({"internal": "123"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("internal"), None);
}

#[test]
async fn internal_field_value_can_be_get_through_get_value() {
    let graph = make_graph().await;
    let simple = graph.create_object("Internal", json!({})).unwrap();
    let _ = simple.set_value("internal", Value::String("123".to_string()));
    let value = simple.get_value("internal").unwrap().unwrap();
    assert_eq!(value, Value::String("123".to_string()));
}
