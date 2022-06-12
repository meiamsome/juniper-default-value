#![allow(dead_code)]

use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

struct Query;

#[graphql_object]
impl Query {
    fn test(#[graphql(default = 100)] arg: Option<i32>) -> Option<i32> {
        arg
    }

    fn non_null_default(#[graphql(default = 100)] arg: i32) -> Option<i32> {
        Some(arg)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

#[cfg(test)]
mod test {
    use super::*;
    use juniper::{DefaultScalarValue, Value};

    #[test]
    fn default_value() {
        let (res, _errors) = juniper::execute_sync(
            "query { test }",
            None,
            &schema(),
            &juniper::Variables::new(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            Some(&100)
        );
    }

    #[test]
    fn literal_number() {
        let (res, _errors) = juniper::execute_sync(
            "query { test(arg: 10) }",
            None,
            &schema(),
            &juniper::Variables::new(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            Some(&10)
        );
    }

    #[test]
    fn literal_null() {
        let (res, _errors) = juniper::execute_sync(
            "query { test(arg: null) }",
            None,
            &schema(),
            &juniper::Variables::new(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            None
        );
    }

    #[test]
    fn variable_field_default_value() {
        let (res, _errors) = juniper::execute_sync(
            "query($arg: Int) { test(arg: $arg) }",
            None,
            &schema(),
            &vec![].into_iter().collect(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            Some(&100)
        );
    }

    #[test]
    fn variable_default_value() {
        let (res, _errors) = juniper::execute_sync(
            "query($arg: Int = 1000) { test(arg: $arg) }",
            None,
            &schema(),
            &vec![].into_iter().collect(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            Some(&1000)
        );
    }
    #[test]
    fn variable_default_value_explicit_null() {
        let (res, _errors) = juniper::execute_sync(
            "query($arg: Int = 1000) { test(arg: $arg) }",
            None,
            &schema(),
            &vec![("arg".to_owned(), Option::<i32>::None.into())]
                .into_iter()
                .collect(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            None
        );
    }

    #[test]
    fn variable_number() {
        let (res, _errors) = juniper::execute_sync(
            "query($arg: Int) { test(arg: $arg) }",
            None,
            &schema(),
            &vec![("arg".to_owned(), 10.into())].into_iter().collect(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            Some(&10)
        );
    }

    #[test]
    fn variable_null() {
        let (res, _errors) = juniper::execute_sync(
            "query($arg: Int) { test(arg: $arg) }",
            None,
            &schema(),
            &vec![("arg".to_owned(), Option::<i32>::None.into())]
                .into_iter()
                .collect(),
            &(),
        )
        .unwrap();

        assert_eq!(
            res.as_object_value()
                .unwrap()
                .get_field_value("test")
                .unwrap()
                .as_scalar_value::<i32>(),
            None
        );
    }

    #[test]
    fn schema_for_default_non_null_is_non_null() {
        let (res, _errors) = juniper::execute_sync(
            r"
                query {
                    __schema {
                        queryType {
                            fields {
                                name
                                args {
                                    name
                                    type {
                                        name
                                        kind
                                        ofType {
                                            name
                                            kind
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            ",
            None,
            &schema(),
            &juniper::Variables::new(),
            &(),
        )
        .unwrap();

        let expected: Value<DefaultScalarValue> = Value::Object(
            vec![("__schema", Value::Object(
                vec![("queryType", Value::Object(
                    vec![("fields", Value::List(vec![
                        Value::Object(
                            vec![
                                ("name", Value::Scalar(DefaultScalarValue::String("test".to_string()))),
                                ("args", Value::List(vec![
                                    Value::Object(
                                        vec![
                                            ("name", Value::Scalar(DefaultScalarValue::String("arg".to_string()))),
                                            ("type", Value::Object(
                                                vec![
                                                    ("name", Value::Scalar(DefaultScalarValue::String("Int".to_string()))),
                                                    ("kind", Value::Scalar(DefaultScalarValue::String("SCALAR".to_string()))),
                                                    ("ofType", Value::Null),
                                                ]
                                                        .into_iter()
                                                        .collect()
                                            ))]
                                                .into_iter()
                                                .collect()
                                    ),
                                ]))]
                                    .into_iter()
                                    .collect()
                        ),
                        Value::Object(
                            vec![
                                ("name", Value::Scalar(DefaultScalarValue::String("nonNullDefault".to_string()))),
                                ("args", Value::List(vec![
                                    Value::Object(
                                        vec![
                                            ("name", Value::Scalar(DefaultScalarValue::String("arg".to_string()))),
                                            ("type", Value::Object(
                                                vec![
                                                    ("name", Value::Null),
                                                    ("kind", Value::Scalar(DefaultScalarValue::String("NON_NULL".to_string()))),
                                                    ("ofType", Value::Object(
                                                        vec![
                                                            ("name", Value::Scalar(DefaultScalarValue::String("Int".to_string()))),
                                                            ("kind", Value::Scalar(DefaultScalarValue::String("SCALAR".to_string()))),
                                                        ]
                                                                .into_iter()
                                                                .collect()
                                                    )),
                                                ]
                                                        .into_iter()
                                                        .collect()
                                            ))]
                                                .into_iter()
                                                .collect()
                                    ),
                                ]))]
                                    .into_iter()
                                    .collect()
                        ),

                    ]))]
                        .into_iter()
                        .collect()
                ))]
                    .into_iter()
                    .collect()
            ))]
                .into_iter()
                .collect(),
        );

        assert_eq!(res, expected);
    }
}
