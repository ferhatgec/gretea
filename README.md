# Fegeya Gretea
## Gretea (aka green tea), new generation programming language.

### A taste of Gretea's syntax:
```rust
import tea.green.fmt

module hello {
    fn hello#display_it<What>(what: What) {
        fmt#println(what)
    }
}

fn main() = int {
  hello#display_it("Hi Gretea!")

  . 0
}
```

### Features:
 * Variables are immutable by default.
 * [Runtime scripting support](https://github.com/ferhatgec/elite)
 * Aliases
 * Compile-time statements.
 * Variadics.
 * C++ codegen backend support.
 * *A bit better* [STL](tea/) called [*tea.green*](tea/green/) (STL) and [*tea.black*](tea/black) (FFI)
 * `compile` keyword for `[[ x ]]` flags (like `default`)

### Gretea licensed under the terms of MIT License.
