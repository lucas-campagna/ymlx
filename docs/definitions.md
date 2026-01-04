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

3. Components can be called with arguments, the result depends on the **component** and **argument** types. The following rules are applied in order:

    1. **argument** is **null**, then **component** value is returned
    2. **component** and **argument** are **sequence**s, the result is the concatenation of **component** with **argument**
    3. **component** or **argument** is **sequence**, the result is a call to each **sequence** item using the other non-**sequence** counterpart as **component** or **argument**
    4. **argument** is **object** and **component** has any variable, the result is the replacement of **argument** variables into **component** variables
    5. **argument** and **component** are **object**s, the result is the result of assignment of **argument** into **component**
    6. **component** contains `_` variable, then the **return** is the pasting of **argument** into all occourences of it or, if it's an `object`, only the remaining variables from step `4.` will be pasted
    7. Otherwise **component** is returned

> A variable is any word starting with dollar sign (`$`) and may contain `_`.
> After replacing variables all unused variables are discarted into `_` the variable (if present).

Example: consider the following components

```yml
comp1: Hello, $name
comp2:
  - a
  - b
  - c
comp3:
  - Hello, $name
  - full_name: $name $last
    year: 2026
  - null
  - 123
comp4:
  a: 1
  b: 2
  c: 3
comp5:
  - $_
  - a: $_
  - The ticket is $_
comp6:
  id: $name
  props: $_
```

- Calling **component** `comp1` with **argument** `{"name": "Alice"}` you get `"Hello, Alice"`.
- Calling **component** `comp2` with **argument** `["d", "e"]` you get `["a", "b", "c", "d", "e"]`.
- Calling **component** `comp3` with **argument** `{"name": "Bob", "last": "Rock"}` you get `["Hello, Bob", {"full_name": "Bob Rock", "year": 2026}, null, 123]`.
- Calling **component** `comp4` with **argument** `$a + $b = $c` you get `"1 + 2 = 3"`.
- Calling **component** `comp4` with **argument** `{"d": 4, "e": 5}` you get `{"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}`.
- Calling **component** `comp5` with **argument** `123` you get `[123, {"a": 123}, "The ticket is 123"]`.
- Calling **component** `comp6` with **argument** `{"name": "Alice", "age": 123, "role": "president"}` you get `{"id": "Alice", "props": {"age": 123, "role": "president"}}`.

3. You can automate component calling by using the `yx-from` property.

Example:

```yml
div: <div>$body</div>
comp:
  yx-from: div
  body: Hello World!
```

When `comp` is called it calls `div` component with `{"body": "Hello World!"}` as argument, which returns `<div>Hello Word!</div>`, then it's returned to the first (`comp`) call.


4. Components can be functions to be called with expected arguments

Example:

```rs
fn sql(args: Props) -> Vec[String] {
  let query = args.query;
  conn.execute(query).unwrap()
}
```

```yml
users:
  yx-from: sql
  query: SELECT name FROM users
```

Calling `users` you get the list of users in your database.

5. You can shortcut the call to other components using it's name with the prefix `yx-`.

> Parameter shortcut rule:
> If **component** has only one variable, or more than one variable but only one starting with `$$` instead of `$`, and **argument** is a scalar (string, number, boolean or null), the applied **argument** is an object with only one key equals to the **component** variable (or that one starting with `$$`) and value equals to the original **argument** scalar.

Example: The example of item `3.` can be shortcuted to

```yml
div: <div>$body</div>
comp:
  yx-div: Hello World!
```

5. template

6. You can eval math in strings using `$()`

Example:

```yml
app: cossine of 60 deg is $(cos(60*pi/180))
```

Calling `app` you get `cossine of 60 deg is 0.5`

7. component pasting

8. regex components


3. To define the component children use the `body` property.

Example:

```yml
box:
  from: div
  body: Hello World!
```

If we render the `box` component, we will get `<div>Hello World!</div>`.

4. Any other property will be added to the tag attribute

Example:

```yml
box:
  from: div
  class: bg-red-100 p-2
  body: Hello World!
```

Will be rendered to `<div class="bg-red-100 p-2">Hello World!</div>`.

5. Any word (with underscore `_`) starting with `$` is considered as property, and can be replaced in the future

Example:

```yml
box:
  from: div
  class: $color p-2
  body: Hello World!
```

> When rendering you can provide the value of `color` and get whatever value do you want.
> You can use as many properties as you want.

6. You can parse strings using `${}`

Example:

```yml
box:
  body: "${$count+1}"
```

Rendering `box` with `count=10` you get `11`.

7. Components can inherit another components using `from`, it overwrites the base component properties

Example:

```yml
box1:
  from: div
  body: Hello World!
box:
  from: box1
  body: World, Hello!
```

Rendering `box` you get `<div>Hello World!</div>`, `box1` you get `<div>World, Hello!</div>`.

8. Inheritance also applies properties

Example:

```yml
box1:
  from: div
  body: Hello, $name!
box:
  from: box1
  name: James
```

Rendering `box` you get `<div>Hello, James!</div>`.

9. Components inside body renders inner components

Example:

```yml
box:
  from: span
  body:
    from: strong
    body: Hello World!
```

Rendering `box` you get `<span><strong>Hello World!</strong></span>`.

10. Bodies can be array

Example:

```yml
box:
  from: span
  body:
    - from: strong
      body: "Hello "
    - from: i
      body: World!
```

Rendering `box` you get `<span><strong>Hello </strong><i>World!</i></span>`.

11. Components with only `body` produces fragmented components

Example:

```yml
box:
  body:
    - from: strong
      body: "Hello "
    - from: i
      body: World!
```

Rendering `box` you get `<strong>Hello </strong><i>World!</i>`.

12. Components can be templated creating a component with same name but starting with `$`

> A template is acts as a base class

Example:

```yml
$box:
  from: div
  body: Hello, $name!
box:
  name: James
```

Rendering `box` you get `<div>Hello, James!</div>`.

13. In fragment templated componets the template is applied to `body`

Example:

```yml
$box:
  from: div
  body: Hello, $name!
box:
  body:
    - name: God
    - name: Adam
    - name: Eve
```

Rendering `box` you get `<div>Hello, God!</div><div>Hello, Adam!</div><div>Hello, Eve!</div>`.

14. You can shorcut fragmented components by omitting the `body` tag:

Example:

```yml
box: ,.\
  - from: div
    body: 1
  - from: span
    body: 2
  - from: p
    body: 3
```

Rendering `box` you get `<div>1</div><span>2</span><p>3</p>`

15. `from` and `body` can be shortcuted by providing a key with the component name and the body as its value

Example:

```yaml
box1: This is a text
box2:
  p: This is a paragraph!
box3:
  - div: First child
  - span: Second child
    class: flex justify-content
  - box1: null
  - box2: New paragraph
```

Rendering `box1` you get `This is a text`.
Rendering `box2` you get `<p>This is a paragraph!</p>`.
Rendering `box3` you get `<div>First child</div><span class="flex justify-content">Second child</span>This is a text<p>New paragraph</p>`.

16. Components can be used in other ones by just providing its name in the body

Example:

```yml
box1: Component 1
box2: Component 2
box3: box1
box4:
  p: box2
box5:
  - box1
  - div: box2
    class: bg-black
  - box4
  - from: box4
    class: bg-red-100
```

Rendering `box3` you get `Component 1`.
Rendering `box4` you get `<p>Component 2</p>`.
Rendering `box5` you get `Component 1<div class="bg-black">Element2</div><p>Component 2</p><p class="bg-red-100">Component 2</p>Final component`.

17. Entry points can be specified using css selectors on keys with format `$(<css-selector>)`

Example:

```yml
$(#root): app
app:
  div: Hello world!
```

It renders `<div>Hello world!</div>` inside the element with id `root`.