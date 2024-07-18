# `surreal-codegen`

Building into binary CLI

```
cargo install --path ./surreal-codegen
```

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
- [ ] Function call parameter inference
- [ ] `SET foo = $bar` parameter inference
- [ ] `CONTENT { foo: $bar }` parameter inference

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


### `CREATE` statements
- [x] `RETURN BEFORE`
- [x] `RETURN AFTER`
- [ ] `RETURN DIFF`
- [x] `RETRUN @statement_param` with `$after` field access


### Value expressions
#### Idiom/path expressions
- [x] `foo.bar`
- [x] `foo.*` for arrays
- [x] `foo.*` for objects
- [ ] `foo[0]`
- [ ] edge traversal eg: `foo->bar<-baz`

#### Literal/constant expressions
- [ ] `true`
- [ ] `false`
- [ ] `null`
- [ ] `"string"`
- [ ] `123`
- [ ] `123.456`
- [ ] `[1, 2, 3]`
- [ ] `{"foo": "bar"}`

#### Comparison expressions
- [ ] `foo == "bar"`
- [ ] `foo != "bar"`
- [ ] `foo < "bar"`
- [ ] `foo <= "bar"`
- [ ] `foo > "bar"`
- [ ] `foo >= "bar"`

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
- [ ] `RETURN`
- [ ] `BEGIN`
- [ ] `COMMIT`
- [ ] `ABORT`
- [ ] `THROW`
