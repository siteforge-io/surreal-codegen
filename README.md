# `surreal-codegen`

Building into binary CLI

```
cargo install --path ./surreal-codegen
```

### General type handling
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
  - [x] `Null` (for `Option`)
- [x] `Either` (mixed return types)


### `SELECT` statements
- [x] All fields
- [x] Fields
- [x] Fields with aliases
- [x] `FROM` targets
- [x] `VALUE`
- [ ] `WHERE` automatic parameter inference
- [ ] `GROUP BY`
- [ ] `SPLIT` fields
- [ ] `FETCH` fields

### `DELETE` statements
- [x] `FROM` targets
- [x] `RETURN BEFORE`
- [x] `RETURN AFTER`
- [ ] `RETURN DIFF`
- [x] `RETRUN @statement_param` with `$before` field access

### `INSERT` statements


### `RELATE` statements


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
#### Idiom expressions
- [x] `foo.bar`
- [x] `foo.*`
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

### Parameter expressions
- [x] custom `$params`
- [ ] built-in parameters
  - [x] `$this`
  - [x] `$parent`
  - [x] `$after`
  - [x] `$before`
  - [ ] `$auth`
  - [ ] `$session`
  - [ ] `$scope`
  - [ ] `$input`
  - [ ] `$token`
- [ ] Automatic parameter inference

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