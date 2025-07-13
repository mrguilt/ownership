# Chapter 4: Understanding Ownership

* Ownership controls memory management.
* Memory is automatically returned once a variable goes out of scope.
    * `drop` function returns memory--happens automatically at a closed curly bracket (`}`)

## The `String` Type

* Used as an example of memory management in this chapter. Looks like more later.
* String literal: `let x="foo"`. Can't be changed. Memory allocated at compile time. 
* String type can be changed. `let s=String::from("foo")`. Can be mutable.
* It does not appear `st="foo";` works to change a string (assuming declared mutable). Need to do `st=String::from("foo");`
    * Googled, this does appear to be the way


