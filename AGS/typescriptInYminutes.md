---
title: Learn TypeScript in Y minutes
date: 2024-11-26
tags: [tag1, tag2]
---

# Learn TypeScript in Y minutes

```TypeScript
// There are 3 basic types in TypeScript
let isDone: boolean = false;
let lines: number = 42;
let name: string = "Anders";

// You can omit the type annotation if the variables are derived from explicit
// literals
let isDone = false;
let lines = 42;
let name = "Anders";

// When it's impossible to know, there's an "Any" type
let notSure: any = 4;
notSure = "maybe a string instead";
notSure = false; // okay, definitely a boolean

// Use const keyword for constants
const numLivesForCat = 9;
numLivesForCat = 1; // Error!

// For collections, there are typed arrays and generic arrays
let list: number[] = [1, 2, 3];
// Alternatively, using the generic array type
let list: Array<number> = [1, 2, 3];

// For enumerations:
enum Color { Red, Green, Blue };
let c: Color = Color.Green;
console.log(Color[c]); // "Green"

// Lastly, "void" is used in the special case of a function returning nothing
function bigHorribleAlert(): void {
  alert("I'm a little annoying box!");
}
```
