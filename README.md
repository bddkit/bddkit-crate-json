# bddkit-crate-json

The JSON path reader and matcher behind [bddkit](https://github.com/bddkit/bddkit)'s JSON assertions. The core and the `bddkit-api` plugin both depend on this crate, so a JSON check means the same thing everywhere.

## `contains` is loose, `equals` is strict

This is deliberate, and a change to it is a breaking change.

- **`matcher::contains`** is a subset match. An object may have extra fields. An array holds every expected element somewhere, in any order, and may have extra elements (the semantics of Imbo, on purpose). Do not tighten it: strict comparison is `equals`, a separate function.
- **`matcher::equals`** is exact. The same keys, the same length, element by element in order.
- **`matcher::not_contains`** is the negative of `contains`, with the same looseness.

The tests in `src/lib.rs` and `src/matcher.rs` pin both rules.

## Matchers

A string in the expected JSON of the form `@name(args)` is a matcher, not a literal:

- `@variableType(string|int|float|bool|array|object|null)` checks the type of the value.
- `@arrayLength(N)` checks the length of an array.
- `@regExp(/pattern/)` checks a string against a regular expression.

Any other `@name(...)` is reported as an unknown matcher. A string that is not of that form, such as `@home`, is compared as is.

## Paths

`path::read` reads a value by a path like `a.b[0].c`, with an optional `root.` prefix. `path::validate` checks the syntax alone, without a document.

## Errors

The crate has its own error types and knows nothing of bddkit's failure classes: [`Mismatch`], [`UnexpectedMatch`], [`PathError`] and [`InvalidJson`]. The caller decides what class a failure has.

## Releases

release-plz publishes from `main` with `semver_check` on, and writes the CHANGELOG. A change to the matcher takes two pull requests: a release of this crate, then the bump of the dependency in bddkit. To work on both at once, point bddkit at a local checkout with a `[patch.crates-io]` entry and never commit it.

## License

Apache-2.0.
