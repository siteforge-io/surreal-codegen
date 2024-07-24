# `surreal-codegen`
> [!WARNING]
> This is a work in progress, but we are currently using it in production at [Siteforge](https://siteforge.io)

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

```sh
surreal-codegen --schema ./schema.surql --query ./query.surql
```

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
