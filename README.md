# swc-ski

This swc plugin transforms Ski component files to native DOM statements/expressions to minimize DOM updates and allow for fine-grained reactivity. This approach is **heavily** inspired by [Ryan Carniato's dom-expressions](https://github.com/ryansolid/dom-expressions).

## progress

- [ ] inspection phase
  - [ ] attributes
    - [ ] boolean (short form)
    - [ ] literals
    - [ ] jsx element
    - [ ] jsx fragment
    - [ ] spreads
    - [ ] expressions
      - [ ] static
      - [ ] dynamic
  - [ ] children/content
    - [ ] text (remove unneccesary whitespace)
    - [ ] jsx element
    - [ ] jsx fragment
    - [ ] spreads
    - [ ] expressions
      - [ ] static
      - [ ] dynamic
- [ ] code generation phase
  - [ ] template declaration
  - [ ] element/children declarations
  - [ ] expressions (setAttr, insert, addEventListener, etc.)
  - [ ] dynamics (also expressions but wrapped in effects)

## improvements

- [ ] try to statically evaluate certain expressions to minimize setAttribute calls (and thus DOM updates)
- [ ] traverse code in attribute expression to identify when it's a dynamic (will be wrapped in an effect)

## testing

```shell
cargo +nightly test
```
