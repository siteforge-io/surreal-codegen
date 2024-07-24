# `surreal-codegen`
> [!WARNING]
> This is a work in progress, but we are currently using it in production at [Siteforge](https://siteforge.io) to help ensure type safety in our SurrealDB queries.

# Installation
> [!NOTE]
> Currently I haven't published this as a easily installable dependency, so you will need to `git clone` this repo and build it yourself.

1. Clone this repo
```sh
git clone https://github.com/siteforge-io/surreal-codegen.git
```

2. Build the binary
```sh
cargo install --path ./surreal-codegen
```

3. Run the binary

```sh
surreal-codegen --help
```

```
Usage: surreal-codegen [OPTIONS] --dir <DIR> --schema <SCHEMA>

Options:
  -d, --dir <DIR>        The directory containing the Surql files
  -s, --schema <SCHEMA>
  -o, --output <OUTPUT>  The name of the output file default of `types.ts` [default: ./types.ts]
  -h, --help             Print help
```

# Usage

## Schema Example

`./schema.surql`
```ts
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string
  VALUE string::lowercase($value)
  ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string
  VALUE crypto::bcrypt::generate($value);
DEFINE FIELD name ON user TYPE string
  VALUE string::trim($value);
DEFINE FIELD created_at ON user TYPE datetime
  VALUE time::now()
  READONLY;
```

## Query Example

`./queries/create_user.surql`
```ts
CREATE user CONTENT $user;
```


## Codegen
This wil generate a `types.ts` file in the current directory, which includes all your queries, as well as some prototype and type overrides for the SurrealDB database to allow you to use the generated types in your TypeScript code.
```sh
surreal-codegen \
  --schema ./schema.surql \
  --dir ./queries \
  --output ./surreal_queries.ts
```

## TypeScript usage
```ts
import { Surreal } from "surreal.js";
import { CreateUserQuery } from "./surreal_queries"

const db = new Surreal({
  ...
});

/*
  Result is typed as CreateUserResult from the generated types.ts file
*/
const result = await db.typed(CreateUserQuery, {
  user: {
    name: "John Doe",
    email: "john@doe.com",
    password: "123456",
  } // can also be an array of users
});
```

# Notes
- We only currently support SCHEMAFULL tables so far, but we are working on supporting other table types.


# Features Supported So Far

### General Type Support and Handling
- [x] `Never`
- [x] `Unknown`
- [x] `String`
- [x] `Int`
- [x] `Float`
- [x] `Datetime`
- [x] `Duration`
- [x] `Decimal`
- [x] `Bool`
- [x] `Record`
- [x] `Option`
- [x] `Array`
- [x] `Object`
- [x] `Number`
- [x] `Null` (for `Option`)
- [x] `Any`
- [x] `None`
- [x] `Either` (mixed return types)

## Objects
- [x] `RETURN { foo: 1, bar: 2 }`

## Automatic Parameter Inference
- [ ] `WHERE foo = $bar` parameter inference
- [ ] `fn::foo($bar)` parameter inference
- [ ] `CREATE baz SET foo = $bar` parameter inference
- [ ] `CREATE baz CONTENT { foo: $bar }` parameter inference
- [x] `CREATE baz CONTENT $foo` parameter inference
- [ ] `UPDATE baz SET foo = $bar` parameter inference
- [ ] `UPDATE baz CONTENT $foo` parameter inference
- [ ] `UPDATE baz MERGE $foo` parameter inference
- [ ] `UPDATE baz MERGE { foo: $bar }` parameter inference
- [ ] `UPSERT baz SET foo = $bar` parameter inference

### `SELECT` statements
- [x] All fields
- [x] Fields
- [x] Fields with aliases
- [x] `FROM` targets
- [x] `VALUE`
- [x] `GROUP BY`
- [x] `GROUP ALL`
- [ ] `SPLIT` fields
- [ ] `FETCH` fields

### `DELETE` statements
- [x] `FROM` targets
- [x] `RETURN BEFORE`
- [x] `RETURN AFTER`
- [ ] `RETURN DIFF`
- [x] `RETRUN @statement_param` with `$before` field access

### `INSERT` statements
- [ ] TODO

### `RELATE` statements
- [ ] TODO

### `DEFINE TABLE .. AS` precomputed tables
- [X] `DEFINE TABLE foo AS SELECT ... FROM bar`
- [X] `DEFINE TABLE foo AS SELECT ... FROM bar GROUP BY ...`
- [X] `DEFINE TABLE foo AS SELECT ... FROM bar GROUP ALL`


### `UPDATE` statements
- [x] `RETURN BEFORE`
- [x] `RETURN AFTER`
- [ ] `RETURN DIFF`
- [x] `RETRUN @statement_param` with `$before` and `$after` field access
- [ ] `CONTENT { foo: $bar }` parameter inference
- [ ] `CONTENT $foo` parameter inference
- [ ] `SET foo = $bar` parameter inference
- [ ] `MERGE $bar` parameter inference
- [ ] `MERGE { foo: $bar }` parameter inference


### `CREATE` statements
- [x] `RETURN BEFORE`
- [x] `RETURN AFTER`
- [ ] `RETURN DIFF`
- [x] `RETRUN @statement_param` with `$after` field access
- [ ] `CONTENT { foo: $bar }` parameter inference
- [ ] `CONTENT $foo` parameter inference
- [ ] `SET foo = $bar` parameter inference

### `UPSERT` statements
- [ ] TODO


### Value expressions
#### Idiom/path expressions
- [x] `foo.bar`
- [x] `foo.*` for arrays
- [x] `foo.*` for objects
- [ ] `foo[0]`
- [ ] edge traversal eg: `foo->bar<-baz`

#### Literal/constant expressions
- [x] `true`
- [x] `false`
- [x] `null`
- [x] `"string"`
- [x] `123`
- [x] `123.456`
- [ ] `[1, 2, 3]`
- [x] `{"foo": "bar"}`

#### Comparison expressions
- [x] `foo == "bar"`
- [x] `foo != "bar"`
- [x] `foo < "bar"`
- [x] `foo <= "bar"`
- [x] `foo > "bar"`
- [x] `foo >= "bar"`

#### Subquery expressions
- [x] `SELECT` statements
- [x] `DELETE` statements
- [ ] `INSERT` statements
- [x] `UPDATE` statements
- [x] `CREATE` statements
- [ ] `RELATE` statements

### Parameter expressions
- [x] Custom global `$param` definitions in a `global.surql` file
  - [x] `$auth`
  - [x] `$session`
  - [x] `$scope`
  - [x] `$input`
  - [x] `$token`
- [-] built-in parameters
  - [x] `$this`
  - [x] `$parent`
  - [x] `$after`
  - [x] `$before`
  - [ ]
- [ ] Automatic parameter inference in some cases

### Other Statements
- [ ] `IF ELSE`
- [ ] `FOR`
- [ ] `CONTINUE`
- [ ] `BREAK`
- [x] `RETURN`
- [ ] `BEGIN`
- [ ] `COMMIT`
- [ ] `LET`
- [ ] `ABORT`
- [ ] `THROW`
