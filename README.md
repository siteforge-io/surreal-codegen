# `surreal-codegen`


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
- [x] Subqueries
- [x] `FROM` targets
- [ ] Value expressions
- [ ] `VALUE`
- [ ] `WHERE` conditions
- [ ] `GROUP BY` fields
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


### `CREATE` statements


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
- [ ]