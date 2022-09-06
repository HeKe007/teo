use actix_http::body::BoxBody;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serial_test::serial;
use actix_web::{test, web, App, error::Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use teo::core::graph::Graph;
use serde_json::json;
use teo::app::app::ServerConfiguration;
use teo::app::serve::make_app;
use crate::helpers::{request, request_get, assert_json_response};

async fn app() -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    let graph = Graph::new(|g| {
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_query_group_by");
        g.reset_database();
        g.model("Record", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("country", |f| {
                f.required().string();
            });
            m.field("city", |f| {
                f.optional().string();
            });
            m.field("profileViews", |f| {
                f.required().i64();
            });
        });
    }).await;
    make_app(graph, ServerConfiguration::default())
}

#[test]
#[serial]
async fn group_by_returns_empty_array_if_no_record() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "records", "group-by", json!({
        "by": ["country"],
    })).await;
    assert_json_response(res, 200, json!({
        "data": []
    })).await;
}

#[test]
#[serial]
async fn group_by_returns_grouped_value_for_single_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Washington",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Los Angeles",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let _id3 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "UK",
            "city": "London",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let res = request(&app, "records", "group-by", json!({
        "by": ["country"],
        "_sum": {
            "profileViews": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": [
            {
                "country": {"equals": "UK"},
                "_sum": {
                    "profileViews": {"equals": 5000}
                }
            },
            {
                "country": {"equals": "US"},
                "_sum": {
                    "profileViews": {"equals": 10000}
                }
            }
        ]
    })).await;
}

#[test]
#[serial]
async fn group_by_returns_grouped_value_for_multiple_fields() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Washington",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Washington",
            "profileViews": 78
        },
    }), 200, "data.id").await;
    let _id3 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Los Angeles",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let _id4 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "UK",
            "city": "London",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let res = request(&app, "records", "group-by", json!({
        "by": ["country", "city"],
        "_sum": {
            "profileViews": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": [
            {
                "country": {"equals": "UK"},
                "city": {"equals": "London"},
                "_sum": {
                    "profileViews": {"equals": 5000}
                }
            },
            {
                "country": {"equals": "US"},
                "city": {"equals": "Los Angeles"},
                "_sum": {
                    "profileViews": {"equals": 5000}
                }
            },
            {
                "country": {"equals": "US"},
                "city": {"equals": "Washington"},
                "_sum": {
                    "profileViews": {"equals": 5078}
                }
            }
        ]
    })).await;
}

#[test]
#[serial]
async fn group_by_returns_null_for_field_value_if_value_is_null_or_not_exist() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Washington",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": null,
            "profileViews": 78
        },
    }), 200, "data.id").await;
    let _id3 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "US",
            "city": "Los Angeles",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let _id4 = request_get(&app, "records", "create", json!({
        "create": {
            "country": "UK",
            "city": "London",
            "profileViews": 5000
        },
    }), 200, "data.id").await;
    let res = request(&app, "records", "group-by", json!({
        "by": ["country", "city"],
        "_sum": {
            "profileViews": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": [
            {
                "country": {"equals": "UK"},
                "city": {"equals": "London"},
                "_sum": {
                    "profileViews": {"equals": 5000}
                }
            },
            {
                "country": {"equals": "US"},
                "city": {"is": "null"},
                "_sum": {
                    "profileViews": {"equals": 78}
                }
            },
            {
                "country": {"equals": "US"},
                "city": {"equals": "Los Angeles"},
                "_sum": {
                    "profileViews": {"equals": 5000}
                }
            },
            {
                "country": {"equals": "US"},
                "city": {"equals": "Washington"},
                "_sum": {
                    "profileViews": {"equals": 5000}
                }
            }
        ]
    })).await;
}
