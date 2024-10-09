# Technical Screen (Otta)

Implement a solution for the instructions [found here](https://thoughtfulautomation.notion.site/Platform-Technical-Screen-b61b6f6980714c198dc49b91dd23d695)

Two solutions have been implemented:

- **simple**: 
  - provides a minimal implementation within the time constraints of the exercise
- **extensive**: 
  - provides a solution adhering to the principles of DDD
  - models all concepts present in the language used to describe the problem

Which solution would be choosen in practice depends on a few factors:

- Does the team have experience with DDD?
- Does the extensive solution provide readability advantages?
- Has a simple solution been exhausted and found lacking?

For this exercise, the simple solution would be preferred, but might 
evolve into the extensive one as the code is maintained. An even simpler
solution could be implemented by only using `usize` types and 
conditional logic contained within a single function. This approach was
first attempted, but did not express the problem and solution clearly enough.

## Instructions

To test the simple code, run:

```shell
cd simple
cargo test
```

Similarly, to test the extensive code, run:

```shell
cd extensive
cargo test
```

## Approach (extensive)

### Testing

The solution has been thoroughly tested. The use of property based
testing ensures that edge cases are covered. One such edge case that
was discovered was an addition of `usize` values that would overflow
when multiple were set to their `usize::MAX` value. This was fixed by
using saturating addition. A regular unit test might not have used these 
extreme values.

Some extreme values, for example negative numbers, are not covered
by testing because the typing system already protects against them.

In particular, the generation of `Arbitrary` values as inputs for testing
protects against an engineer forgetting to test a particular edge case.

### Code quality

Quality of code is subjective, but here are some of the considerations I've
made:

- are the problem concepts clearly expressed?
  - I've chosen to avoid booleans in favor of `enum` types that clarify the
    intent of the code
  - I've chosen to use the domain language from the assignment, instead of
    creating new terms
- what elements should be public vs private?
  - only the `safe_sort()` function is public, including its return types `SortResult` and `SortError`
  - this simplifies refactoring in the future as there is less potential for breaking changes
- Are the public elements documented?
- Are the public elements fully tested?

### Edge case handling

See the testing section for how edge cases are handled. The code uses
property based testing with the generation of random valid or invalid 
inputs.

Another way of edge case handling is to choose the types sufficiently
strict.

### Test coverage

The code is fully covered by tests as evidenced by the `cargo tarpaulin`
command. Full code line coverage is not necessary full semantic coverage,
but attention was given to this as well.

### Future improvements

More of the domain could be expressed. Separate types could be 
created for `Width`, `Height` and `Length`.  `CmError` and `KgError`
could be converted to `WidthError`, `HeightError`, `LengthError` and
`MassError`. These new error types could then have `From` implementations
to allow for easy conversion up until the `SortError` type. This would
remove the manual error conversions in the `sort` function.

## Approach (simple)

Mostly similar explanation of the approach, but with less domain detail.
The type names are also slightly different.

The publicly documented elements are:

- the `sort()` function