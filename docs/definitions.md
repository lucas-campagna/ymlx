# ymx

YMX is a component parser system, using a declarative language.

## Definitions

1. Every component is a key/value pair item in the main body.

Example:

```yml
comp: Hello World!
```

2. You can call a component from its key (name) and you get its value:

Example: calling `comp` from previous example you get the string `"Hello World!"`.

3. Components are called with properties by passing an `object` as unique argument. To access the properties you use `$<property_name>` (`$<property_name>` matches `\$w+`)

```yml
comp: Hello $who
```

Calling `comp` with `{"who": "World"}` you get `"Hello World"`

4. The `body` property maps to the hole argument.

Example:

```yml
comp: Hello $body
```

Calling `comp` with:
  - `"World"` -> `"Hello World"`
  - `123` -> `"Hello 123"`
  - `["W", "o", "r", "l", "d"]` -> `"Hello ["W", "o", "r", "l", "d"]"`
  - `{"who": "World"}` -> `"Hello {"who": "World"}"`

> You could write

```yml
comp: Hello ${body.who}
```

And call it with `{"who": "World"}` so you get the same result.

5. You can do more complex thigs inside processing contexts `${` and `}`.

Example:

```yml
comp: ${a + b}
```

Calling `comp` with `{"a": 1, "b": 2}` you get `3`

> Because components are just functions you can call them inside `${` and `}`:

Example:

```yml
fibo: "${n < 2 ? 1 : n + fibo(n - 1)}"
main: Math is cool because fibo of 10 is ${fibo({"n":10})}
```

Calling `main` you get `"Math is cool because fibo of 10 is 55"`

> The available commands and code format running inside `${` and `}` depends on the interpreter you choose before hand, on the example above we choose [Boa js](https://boajs.dev/) but you could choose [RustPython](https://rustpython.github.io/) to run Python.
> In fact, the above `$<property_name>` is just a shortcut for `${property_name}`, before the yaml string is parsed all `\$(\w+)` is rewriten to `\${\1}`, where `\1` is the match content inside parentesis.

6. You can automate component calling by using the `from!` or `yx-from` or `From` property.

Example:

```yml
div: <div>$body</div>
comp:
  from!: div
  body: Hello World!
```

When `comp` is called it calls `div` component with `{"body": "Hello World!"}` as argument, which returns `<div>Hello Word!</div>`, then it's returned to the first (`comp`) call.

7. Components can be functions to be called with expected arguments

Example:

```rs
fn sql(args: Props) -> Vec[String] {
  let query = args.query;
  conn.execute(query).unwrap()
}
```

```yml
sql:
  from!: sql
  lib: /path/to/lib.dll
  args:
    - query
Users:
  From: sql
  query: SELECT name FROM users
```

Calling `Users` you get the list of users in your database.

8. You can shortcut the call to other components using it's name with the prefix `yx-` or appending `!` at the end or just writing it with first letter in uppercase.

Example: The example of item `6.` can be shortcuted to

```yml
div: <div>$body</div>
comp:
  div!: Hello World!
```

The same result could be obtained with

```yml
div: <div>$body</div>
comp:
  yx-div: null
  body: Hello World!
```

In the first example `div` is being called with `"Hello World!"` as argument, in the second one with `{"body": null, "body": "Hello World!"}`.

Example:

```yml
div: <div class="$class" $action>$body</div>
comp:
  yx-div: Hello World!
  class: bg-red-500
  action: onclick="alert("clicked!")"
```

<!-- 9. Call to components can be simplified in the caller component name by using the caller betewen brackets `()`. -->
<!---->
<!-- Example: The last example can be written as -->
<!---->
<!-- ```yml -->
<!-- div: <div>$body</div> -->
<!-- comp(div): Hello World! -->
<!-- ``` -->

9. You can merge components by appending a component name between brackets `(` and `)` to the end of the component name.

Example:

```yml
```
comp_c:
  c: 3
comp_b:
  b: 1
  c: 10
comp_a(comp_b,comp_c):
  a: 1
  b: 2

Because both `comp_a`, `comp_b` and `comp_c` are objects, the merging result is the [`assing`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign) of `comp_a` into `comp_b`, which produces `{"a": 1, "b":2, "c": 10}`, then the result into `comp_c` which produces `{"a": 1, "b": 2, "c": 3}`.

> TODO: needs explain how do merging is for different types

10. You can hide implementation details by creating template components. The template of a component is a component with the same name as it's derived component but starting with a `$`. The template component is append to the derived component merging list.

Example:

Doing this

```yml
$app:
  age: ${2026 - year_of_birth}
app:
  name: $name
```


Is the same as doing this:

```yml
comp:
  age: ${2026 - year_of_birth}
app(comp):
  name: $name
```
```


!-- # Mapping -->
<!-- b: -->
<!--   - ${x + y} -->
<!--   - ${x * y} -->
<!---->
<!-- a(b): -->
<!--   x: 1 -->
<!--   y: 2 -->
<!---->
<!-- #a -> ["1 + 2", "1 * 2"] -->
<!---->
<!-- --- -->
<!-- # Reducing -->
<!-- b: -->
<!--   x: 1 -->
<!--   y: 2 -->
<!---->
<!-- a(b): -->
<!--   - ${x + y} -->
<!--   - z: $default -->
<!--     x: $default + 1 # 3 + 1 -->
<!--   - ${z + (x * y)} -->
<!---->
<!-- # a -> (1 + 2) + (4 * 2) -->

