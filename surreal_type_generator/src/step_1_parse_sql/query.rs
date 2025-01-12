use std::collections::BTreeMap;

use surrealdb::sql::{parse, Cast, Param, Statement, Value};

pub struct QueryParsed {
    pub statements: Vec<Statement>,
    pub casted_parameters: BTreeMap<String, crate::Kind>,
}

pub fn parse_query(query: &str) -> Result<QueryParsed, anyhow::Error> {
    // collect and filter out all the variable castings
    let mut parameter_types = BTreeMap::new();
    let mut statements = Vec::new();

    for stmt in parse(query)?.into_iter() {
        match stmt {
            Statement::Value(Value::Cast(box Cast {
                0: kind,
                1: Value::Param(Param { 0: ident, .. }),
                ..
            })) => {
                parameter_types.insert(ident.to_string(), kind);
            }
            _ => statements.push(stmt),
        }
    }

    Ok(QueryParsed {
        statements,
        casted_parameters: parameter_types,
    })
}

/// In surreal 2.0, `RETURN` statements now can "early-exit" in the AST
/// This effectively treats `BEGIN/COMMIT` grouped statements as a block
/// but only if they contain a `RETURN` statement
///
/// So we need to turn and represent the Vec<Statement> into a Vec<Vec<Statement>> representing
/// every branching case and scenario.
///
/// ### Example 1:
/// ```surql
/// BEGIN;
/// CREATE ONLY foo;
/// CREATE ONLY bar;
/// RETURN 1;
/// COMMIT;
/// RETURN 2;
/// ```
///
/// Would return:
/// ```ts
/// 0: <number> (1)
/// 1: <number> (2)
/// ```
///
/// OR
///
/// /// What this effectively means, is that a transaction containing a RETURN statement
/// is effectively converted into a block-like AST, looking like the following:
///
/// ```sql
/// {
///     CREATE ONLY foo;
///     CREATE ONLY bar;
///     RETURN 1;
/// }
/// RETURN 2;
/// ```
///
/// ### Example 2:
///
/// And without a `RETURN`
///
/// ```surql
/// BEGIN;
/// CREATE ONLY foo;
/// CREATE ONLY bar;
/// COMMIT;
/// RETURN 2
/// ```
///
/// Would return a type such as:
/// 0: { id: record<foo> }
/// 1: { id: record<bar> }
/// 2: <number> (2)
///
/// ### Example 3:
/// ```surql
/// BEGIN;
/// IF condition {
///     RETURN 1;
/// }
/// RETURN 2;
/// COMMIT;
/// ```
/// Would turn into an AST looking like the following:
/// ```sql
/// {
///     IF condition {
///         RETURN 1;
///     }
///     RETURN 2;
/// }
/// ```
///
/// ### Example 4:
/// ```surql
/// BEGIN;
///
/// IF condition {
///     RETURN 1;
/// }
///
/// COMMIT;
/// RETURN 2;
/// ```
/// Would turn into an AST looking like the following:
/// ```sql
/// {
///     IF condition {
///         RETURN 1;
///     }
///     RETURN 2;
/// }
/// ```
pub fn statements_to_block_ast(
    statements: Vec<Statement>,
) -> Result<Vec<Statement>, anyhow::Error> {
    Ok(statements)
}
