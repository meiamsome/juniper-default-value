const graphql = require('graphql');

const Query = new graphql.GraphQLObjectType({
    fields: {
        test: {
            type: graphql.GraphQLInt,
            args: {
                "arg": {
                    type: graphql.GraphQLInt,
                    defaultValue: 100,
                },
            },
            resolve: (_, { arg }) => arg,
        },
        nonNullDefault: {
            type: graphql.GraphQLInt,
            args: {
                "arg": {
                    type: new graphql.GraphQLNonNull(graphql.GraphQLInt),
                    defaultValue: 100,
                },
            },
            resolve: (_, { arg }) => arg,
        },
    },
    name: "Query",
});

const schema = new graphql.GraphQLSchema({
    query: Query,
});


test('default value', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query { test }"),
            variableValues: {},
        })
    ).toEqual({
        data: {
            test: 100,
        },
    });
});


test('literal number', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query { test(arg: 10) }"),
            variableValues: {},
        })
    ).toEqual({
        data: {
            test: 10,
        },
    });
});

test('literal null', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query { test(arg: null) }"),
            variableValues: {},
        })
    ).toEqual({
        data: {
            test: null,
        },
    });
});

test('variable field default value', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query($arg: Int) { test(arg: $arg) }"),
            variableValues: {},
        })
    ).toEqual({
        data: {
            test: 100,
        },
    });
});


test('variable default value', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query($arg: Int = 1000) { test(arg: $arg) }"),
            variableValues: {},
        })
    ).toEqual({
        data: {
            test: 1000,
        },
    });
});


test('variable default value explicit null', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query($arg: Int = 1000) { test(arg: $arg) }"),
            variableValues: {
                arg: null,
            },
        })
    ).toEqual({
        data: {
            test: null,
        },
    });
});

test('variable number', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query($arg: Int) { test(arg: $arg) }"),
            variableValues: {
                arg: 10,
            },
        })
    ).toEqual({
        data: {
            test: 10,
        },
    });
});

test('variable null', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse("query($arg: Int) { test(arg: $arg) }"),
            variableValues: {
                arg: null,
            },
        })
    ).toEqual({
        data: {
            test: null,
        },
    });
});


test('schema for default non null is non null', () => {
    expect(
        graphql.executeSync({
            schema,
            document: graphql.parse(`
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
            `),
            variableValues: {},
        })
    ).toEqual({
        data: {
            __schema: {
                queryType: {
                    fields: [
                        {
                            name: "test",
                            args: [{
                                name: "arg",
                                type: {
                                    kind: "SCALAR",
                                    name: "Int",
                                    ofType: null,
                                },
                            }],
                        },
                        {
                            name: "nonNullDefault",
                            args: [{
                                name: "arg",
                                type: {
                                    kind: "NON_NULL",
                                    name: null,
                                    ofType: {
                                        kind: "SCALAR",
                                        name: "Int",
                                    },
                                },
                            }],
                        },
                    ],
                },
            },
        },
    });
});
