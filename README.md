# Juniper vs graphql-js default value handling

See https://github.com/graphql-rust/juniper/issues/1073

## Test Results:

graphql-js:
```
> jest index.test.js

 PASS  ./index.test.js
  ✓ default value (3 ms)
  ✓ literal number (1 ms)
  ✓ literal null
  ✓ variable field default value
  ✓ variable default value
  ✓ variable default value explicit null
  ✓ variable number
  ✓ variable null
  ✓ schema for default non null is non null (1 ms)
```

juniper:
```
running 9 tests
test test::variable_number ... ok
test test::variable_default_value ... ok
test test::default_value ... ok
test test::variable_field_default_value ... ok
test test::literal_null ... FAILED
test test::variable_null ... FAILED
test test::literal_number ... ok
test test::variable_default_value_explicit_null ... FAILED
test test::schema_for_default_non_null_is_non_null ... FAILED

failures:

---- test::literal_null stdout ----
thread 'test::literal_null' panicked at 'assertion failed: `(left == right)`
  left: `Some(100)`,
 right: `None`', src/lib.rs:82:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- test::variable_default_value_explicit_null stdout ----
thread 'test::variable_default_value_explicit_null' panicked at 'assertion failed: `(left == right)`
  left: `Some(100)`,
 right: `None`', src/lib.rs:146:9

---- test::variable_null stdout ----
thread 'test::variable_null' panicked at 'assertion failed: `(left == right)`
  left: `Some(100)`,
 right: `None`', src/lib.rs:190:9

---- test::schema_for_default_non_null_is_non_null stdout ----
thread 'test::schema_for_default_non_null_is_non_null' panicked at 'assertion failed: `(left == right)`
  left: `Object(Object { key_value_list: {"__schema": Object(Object { key_value_list: {"queryType": Object(Object { key_value_list: {"fields": List([Object(Object { key_value_list: {"name": Scalar(String("test")), "args": List([Object(Object { key_value_list: {"name": Scalar(String("arg")), "type": Object(Object { key_value_list: {"name": Scalar(String("Int")), "kind": Scalar(String("SCALAR")), "ofType": Null} })} })])} }), Object(Object { key_value_list: {"name": Scalar(String("nonNullDefault")), "args": List([Object(Object { key_value_list: {"name": Scalar(String("arg")), "type": Object(Object { key_value_list: {"name": Scalar(String("Int")), "kind": Scalar(String("SCALAR")), "ofType": Null} })} })])} })])} })} })} })`,
 right: `Object(Object { key_value_list: {"__schema": Object(Object { key_value_list: {"queryType": Object(Object { key_value_list: {"fields": List([Object(Object { key_value_list: {"name": Scalar(String("test")), "args": List([Object(Object { key_value_list: {"name": Scalar(String("arg")), "type": Object(Object { key_value_list: {"name": Scalar(String("Int")), "kind": Scalar(String("SCALAR")), "ofType": Null} })} })])} }), Object(Object { key_value_list: {"name": Scalar(String("nonNullDefault")), "args": List([Object(Object { key_value_list: {"name": Scalar(String("arg")), "type": Object(Object { key_value_list: {"name": Null, "kind": Scalar(String("NON_NULL")), "ofType": Object(Object { key_value_list: {"name": Scalar(String("Int")), "kind": Scalar(String("SCALAR"))} })} })} })])} })])} })} })} })`', src/lib.rs:301:9


failures:
    test::literal_null
    test::schema_for_default_non_null_is_non_null
    test::variable_default_value_explicit_null
    test::variable_null

test result: FAILED. 5 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
