# DATA_STRUCTURES

## Content:
* [Dictonary](#dictonary)
  * a list of key-data pairs

---
## Dictonary

### Explanation:
The Dictonary is a data structur, that is containing key-data pairs. That means, that you need the key of the pair to get the data. Like you look in a dictonary for the translation of bottle (key) in german. In this example the return value (data) would be the word Flasche.

### Turtorial:
#### Chapter:
1. [creating a dictonary](#creating-a-dictonary)
2. [first steps (dictonary)](#first-steps-dictonary)
   1. [adding pair]()
   2. remove pair
   3. clearing dictonary
3. read from a dictonary
   1. geting data
   2. geting pair
4. changing a dictonary
   1. change data
   2. change key


#### Creating a dictonary
To create a dictonary you just have to defining a variable and calling the new() method of the Dictonary struct. Like so:
```rust
let dictonary = Dictonary::new();
```
If you try to compile it now, you get the following error:
```bat
error[E0283]: type annotations needed for `Dictonary<D>`
```
That just mean, we have alse to define the type of our dictonary. So let us adding the type of our variable. In this example i use the u8 type. Now it should look lik this:
```rust
let dictonary:Dictonary<u8> = Dictonary::new();
```
And now you have your first dictonary in rust.

#### First steps (dictonary)
**Adding pair**

---