# readmers

A script to create awesome README

To easy mantain the README file, I will use variables at the start of this file. As much variables as I need. For example,

```html
<!-- start params
variable1: valor1
variable2: valor2
variable3:
variable3_hidden: true
# end params -->
```

The `_hidden` suffix indicates that element is hidden or not not. If it isn't present, indicates that is visible by default.

In the README file...

```html
<span id="variable1">valor1</span>
<h1 id="varible1">valor1<h1>
<span id="variable3" hidden></span>
```

In the configuration of `readmers` must be a set of *templates* for readme.

The templates  must be a combination between markdown and html. Html to simplify the fill of the template.

* When there is no README file, and I run `readmers`, must show a selection, so the user cat select the README template. After that, reading the variables, must answer for every variable, and last fill the template and render.
* If the README file exists, must show to options,
    1. Render
    2. Ask for variables
* Must be exists a plugin to generate contributors section
