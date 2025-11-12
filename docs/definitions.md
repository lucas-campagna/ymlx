# htymlx

## Definitions

1. Every component is a key/pair item in the main body.
2. To define the component tag use the `from` property.
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

18. Components can be functions to be called with expected arguments

Example:

```rs
fn sql(query: String) -> Result<String> {
  conn.execute(query)?
}
```

```yml
app:
  ol:
    from: sql
    query: SELECT name FROM users;
    body:
      li: $name
```

Rendering `app` you could get `<ol><li>God</li><li>Adam</li><li>Eve</li></ol>`
