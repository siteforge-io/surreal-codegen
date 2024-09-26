# `surreal-codegen`
> [!WARNING]
> This is a WIP, but we are currently using it in production at [Siteforge](https://siteforge.io) to help ensure type safety in our SurrealDB queries.
> See the [Features Supported](#features-supported) section for a list of features we currently support.

# Installation
> [!WARNING]
> We haven't currently setup a build automation system, so you must build the project yourself.


## Manual Installation
You must have the rust toolchain installed, then run:

```sh
cargo install --git https://github.com/siteforge-io/surreal-codegen.git
```

## Running `surreal-codegen`
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
  --output ./queries.ts
```

## TypeScript usage
```ts
import { TypedSurreal, CreateUserQuery } from "./queries"

const db = new TypedSurreal({
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

## Typing parameters

We exploit the SurrealDB casting system to infer the types of parameters, for places where they cannot be inferred from the query itself.

All you must do is add a casting annotation with the parameter name, eg:

```sql
-- Casting syntax in SurrealDB.
<string> $email;
```

This will allow the codegen to infer the type of `$email` variable as a `string`.

### Example:

`./queries/reset_password.surql`
```sql
<record<user>> $user;
<string> $password;

UPDATE ONLY $user
  SET password = $password
```

### Global parameters
You can also define global parameters in a `global.surql` file, which will be available to all queries in the directory, this is useful things like typing the $auth parameters available in SurrealDB across all queries.

`./queries/globals.surql`
```sql
<record<user>> $user;
```

# Notes
- We only currently support SCHEMAFULL tables so far, but we are working on supporting other table types.


# Features Supported

### General Type Support and Handling
- [x] `never`
- [x] `unknown`
- [x] `string`
- [x] `int`
- [x] `float`
- [x] `datetime`
- [x] `duration`
- [x] `decimal`
- [x] `bool`
- [x] `record<table>`
- [x] `option<type>`
- [x] `array<type>`
- [x] `object`
- [x] `number`
- [x] `NULL`
- [x] `NONE` (for `Option`)
- [x] `any`
- [x] `foo | bar` Unions (mixed return type unions)
- [x] Surreal 2.0 typed literals (eg: `"foo"`, `123`, `1d`, `{ foo: 123 }`, `array<1|2>`)
- [ ] GEOJson types (eg: `point`, `line`, `polygon`)

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
- [X] `RETURN BEFORE`
- [X] `RETURN AFTER`
- [X] `RETURN DIFF`
- [X] `RETRUN @statement_param` with `$after` field access
- [ ] `MERGE { foo: $bar }` parameter inference
- [ ] `MERGE $foo` parameter inference
- [ ] `CONTENT { foo: $bar }` parameter inference
- [X] `CONTENT $foo` parameter inference
- [ ] `SET foo = $bar` parameter inference


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
- [ ] `UPSERT` statements

### Parameter expressions
- [x] Custom global `$param` definitions in a `global.surql` file
  - [x] `$auth`
  - [x] `$session`
  - [x] `$scope`
  - [x] `$input`
  - [x] `$token`
- [ ] built-in parameters
  - [x] `$this`
  - [x] `$parent`
  - [x] `$after`
  - [x] `$before`
- [ ] Automatic parameter inference in some cases

### Other Statements
- [ ] `IF ELSE`
- [ ] `FOR`
- [ ] `CONTINUE`
- [ ] `BREAK`
- [x] `RETURN`
- [X] `BEGIN`
- [X] `COMMIT`
- [ ] `LET`
- [ ] `ABORT`
- [ ] `THROW`
