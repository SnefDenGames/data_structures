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
   1. [adding pair](#adding-pair)
   2. [remove pair](#remove-pair)
   3. [clearing dictonary](#clearing-dictonary)
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

##### Adding pair
To add a pair to an existing dictonary, you just need a key of the type String and the data you want to bind to the key. Therefore we use the `add_pair(k,d)` method of the `Dictonary` struct. The parameter k stands for the key and d for the data. Here an example:
```rust
// creating a mutalbe dictonary
let mut dictonary:Dictonary<u8>	=	Dictonary.new();

// adding a new pair
dictonary.add_pair("key0".to_string(),0_u8);
```
Like you see, we call the `add_pair()` method of the Dictonary struct. Also we give this method a key (String) and some data as parameter.

##### Remove pair
After we can add pairs to the dictonary, we maybe also want do remove some old pairs. Like the explination of the [Dictonary](#dictonary) tell, we ned the key to get to the pair. That means we also need the key to remove it. So at first we create annd fill a dictonary like this:
```rust
// creating a mutalbe dictonary
let mut dictonary:Dictonary<u8>	=	Dictonary.new();

// fill the dictonary
dictonary.add_pair("zero".to_string(),0_u8);
dictonary.add_pair("one".to_string(),1_u8);
dictonary.add_pair("two".to_string(),10_u8);
```
In this example the key should match to the value of the data. But wait, two is not equal to 10. However, we can remove it. Therefore we call the `remove()` method of the `Dictonary` struct. The only parameter is the key. So the call look like this:
```rust
// remove two from dictonary
dictonary.remove("two".to_string());
```

##### Clearing dictonary
But wait, didn't we cleared some pairs from the dictonary? Yes, but the `remove()` method is just for removing one pair at the time. If you need to remove all pairs from a dectonary, you can remove every pair. So let's create the `Dictonary` at first:
```rust
// creating a mutalbe dictonary
let mut dictonary:Dictonary<u8>	=	Dictonary.new();

// fill the dictonary
dictonary.add_pair("zero".to_string(),0_u8);
dictonary.add_pair("one".to_string(),1_u8);
dictonary.add_pair("two".to_string(),2_u8);
dictonary.add_pair("ten".to_string(),10_u8);
dictonary.add_pair("eleven".to_string(),11_u8);
```
So and now let's remove every pair with the `remove()` method:
```rust
// remove all pairs
dictonary.remove("zero".to_string());
dictonary.remove("one".to_string());
dictonary.remove("two".to_string());
dictonary.remove("ten".to_string());
dictonary.remove("eleven".to_string());
```
But we can also do this and using the `clear()` method:
```rust
// clear the dictonary
dictonary.clear();
```
As you may see, you don't need the any keys for this method. This is because the `clear()` method clears all et once.

---