use pretty_assertions_sorted::assert_eq_sorted;
use surreal_type_generator::{Kind, QueryResult};

#[test]
fn return_and_expressions() -> anyhow::Result<()> {
    let query = r#"
-- constant expressions
RETURN "bar";
RETURN math::e;


-- unary expressions
RETURN -1;
RETURN !true;

-- binary comparison expressions
RETURN true && false;
RETURN true == false;
RETURN 1 = 1;
RETURN 1 != 1;
RETURN 1 IS NOT 1;
RETURN 1 < 3;
RETURN 1 > 3;
RETURN 1 is 1;
RETURN 1 <= 1;
RETURN 1 >= 1;
RETURN 1 && 1;
RETURN 1 || 1;

"#;
    let schema = r#"
DEFINE TABLE placeholder SCHEMAFULL;
"#;

    let QueryResult { return_types, .. } =
        surreal_type_generator::step_3_codegen::query_to_return_type(query, schema)?;

    assert_eq_sorted!(
        return_types,
        vec![
            Kind::String,
            Kind::Number,
            Kind::Number,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool,
            Kind::Bool
        ]
    );

    Ok(())
}

/*
-- arithmetic expressions
RETURN 1 + 1;
RETURN 1 - 1;
RETURN 1 * 1;
RETURN 1 ** 1;
RETURN 1 / 1;
RETURN 1 % 2;
*/

/*
-- shortcircuiting expressions
RETURN 1 ?: 1;
RETURN 1 ?: null;
RETURN null ?? 1;
RETURN 1 ?? 1;
*/

// -- ## TODOS ##
// -- !~
// -- ~
// -- ?~
// -- *~
// -- CONTAINS
// -- CONTAINSNOT
// -- CONTAINSALL
// -- CONTAINSANY
// -- CONTAINSNONE
// -- INSIDE
// -- NOTINSIDE
// -- ALLINSIDE
// -- ANYINSIDE
// -- NONEINSIDE
// -- OUTSIDE
// -- INTERSECTS
// -- @@ (MATCHES)
// -- <|4|> #KNN
