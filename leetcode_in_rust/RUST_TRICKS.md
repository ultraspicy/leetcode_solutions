# Rust Tricks & Non-Obvious Patterns

Notes from my LeetCode solutions — patterns worth remembering.

---

## 1. Iterator Adapters

### `.windows(n)` — sliding window over a slice/vec
```rust
// Check if sorted: collect first, then window
.collect::<Vec<_>>().windows(2).all(|pair| pair[0] <= pair[1])

// 3-word sliding window (problem_1078)
text.split_ascii_whitespace().collect::<Vec<_>>().windows(3)
```

### `.step_by(n)` — custom step size on a range
```rust
// Every other odd index (problem_1455)
(start_odd_index..nums.len()).step_by(2)

// Skip first, take every 2nd, limit count (problem_3627)
.skip(1).step_by(2).take(nums.len() / 3).sum()
```

### `.chunk_by(|&x| x)` — group consecutive equal elements (itertools)
```rust
// Count run lengths (problem_1446)
s.chars().chunk_by(|&x| x).into_iter().map(|(_key, group)| group.count())

// Collect each run into a String (problem_925)
s.chars().chunk_by(|&x| x).into_iter().map(|(_key, group)| group.collect::<String>())
```

### `.flat_map()` — flatten nested iterators
```rust
// Repeat each element N times (problem_1002)
(0..alphabet.len()).flat_map(|i| vec![alphabet_char; min_count])
```

### `.fold()` — stateful accumulation
```rust
// Build stack (problem_1047)
s.chars().fold(Vec::new(), |mut acc, ch| { /* push/pop logic */ acc })

// Build frequency map inline (problem_1160)
s.chars().fold(HashMap::new(), |mut acc, ch| {
    *acc.entry(ch).or_insert(0) += 1;
    acc
})
```

### `.find_map()` — find + transform in one step
```rust
// Returns first Some(value) (problem_1455)
words.iter().enumerate().find_map(|(i, &s)| {
    is_prefix(s, word).then(|| (i + 1) as i32)
})
```

### `.chain()` — concat two iterators
```rust
// Combine words from two strings (problem_884)
s1.split_whitespace().chain(s2.split_whitespace())
```

### `.sorted_by()` with `.then_with()` — multi-key sort (itertools)
```rust
// Primary sort reversed, tie-break by node id (problem_3604/3606)
sorted_by(|a, b| {
    get_priority(&b).cmp(&get_priority(&a))
        .then_with(|| a.id.cmp(&b.id))
})
```

---

## 2. HashMap / BTreeMap

### Entry API — atomic get-or-insert
```rust
// Basic (problem_3238)
*map.entry(key).or_insert(0) += 1;

// Nested maps (problem_3238)
map.entry(outer_key).or_insert_with(HashMap::new)
   .entry(inner_key).or_insert(0);
```

### BTreeMap range queries — crucial for interval problems
```rust
// All keys < left, take the last one (problem_715)
self.left_to_right.range(..left).next_back()

// All keys <= left (inclusive)
self.left_to_right.range(..=left).next_back()

// All keys >= right
self.right_to_left.range(right..)

// All keys in [ll, rr]
self.left_to_right.range(ll..=rr)
```

### BTreeMap entry manipulation
```rust
// Mutable access to max entry (problem_716)
self.index_to_number.last_entry()         // OccupiedEntry (mutable)
self.index_to_number.last_key_value()     // (&K, &V) (immutable)
occupied.remove_entry()                   // removes and returns (K, V)
```

---

## 3. String & Char

### Char arithmetic via `as u8`
```rust
// Convert char to 0-based index (problem_3274)
let col = (c as u8 - b'a') as usize;

// Char frequency array (problem_1189)
freq[ch as usize - 'a' as usize] += 1;
```

### String formatting
```rust
format!("{:04}", num)    // zero-padded to 4 digits (problem_3270)
format!("{:b}", number)  // binary string (problem_3280)
```

### `.split_ascii_whitespace()` vs `.split_whitespace()`
```rust
// split_ascii_whitespace is faster (ASCII-only), good for word problems
text.split_ascii_whitespace().collect::<Vec<_>>()
```

### String repeat and join
```rust
" ".repeat(n)                   // "   " (n spaces)
words.join(&" ".repeat(spaces)) // joins with multi-space separator (problem_1592)
```

### Mutable char vec for in-place editing
```rust
// Can't index String directly — collect to Vec<char> first (problem_1576)
let mut chars: Vec<char> = s.chars().collect();
chars[i] = '*';
chars.iter().collect::<String>()
```

### Case-insensitive char comparison
```rust
ch.to_ascii_lowercase() == other.to_ascii_lowercase()
```

### Substring via `.get(range)`
```rust
str1.get(0..gcd_len).unwrap().to_string()  // (problem_1071)
```

---

## 4. Option & Result

### `.copied()` / `.cloned()` on `Option<&T>`
```rust
// HashMap::get returns Option<&V>; use these to get Option<V>
map.get(&key).copied()   // for Copy types (i32, etc.)
map.get(&key).cloned()   // for Clone types (String, Rc, etc.)
```

### `.then()` — bool → Option
```rust
// Returns Some(value) if true, None if false
(!sources.contains(&p[1])).then(|| p[1].clone())
condition.then(|| index + 1)
```

### `.map_or(default, |v| expr)` — unwrap with transform and fallback
```rust
v.values().max().map_or(false, |&max| max > k)
```

### `?` operator for error propagation
```rust
// In a function returning Result<_, _> (problem_3280)
let number = s.parse::<u32>()?;
```

---

## 5. Smart Pointers: `Rc<RefCell<T>>` + `Weak`

Used for doubly-linked lists, skip lists, and other graph-like structures.

```rust
// Node definition (problem_146 LRU Cache)
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,  // Weak avoids reference cycles
}

// Creating a node
let node = Rc::new(RefCell::new(Node { val: 0, ..Default::default() }));

// Cloning Rc (bumps ref count, doesn't clone contents)
let node_ref = Rc::clone(&node);

// Immutable borrow
let v = node.borrow().val;

// Mutable borrow
node.borrow_mut().next = Some(other_node.clone());

// Downgrade Rc → Weak
node.borrow_mut().prev = Some(Rc::downgrade(&parent));

// Upgrade Weak → Option<Rc>
if let Some(strong) = weak_ref.upgrade() {
    strong.borrow_mut().next = ...;
}
```

**Key rule**: Use `Weak` for "back" pointers (parent, prev) to break cycles.

---

## 6. Custom `Ord` for `BinaryHeap`

Rust's `BinaryHeap` is a max-heap. To customize ordering:

```rust
// Min-heap by wrapping with std::cmp::Reverse (problem_3607)
use std::cmp::Reverse;
heap.push(Reverse(val));

// Custom struct with manual Ord (problem_3604)
#[derive(Eq, PartialEq)]
struct State { min_time: i32, node: i32 }

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_time.cmp(&self.min_time)  // reversed = min-heap
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for State { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
```

---

## 7. Closures Capturing Environment

```rust
// Closure captures and mutates an outer iterator (problem_917)
let mut iter_alpha = s.chars().rev().filter(|c| c.is_ascii_alphabetic());
s.chars().map(|c| {
    if c.is_ascii_alphabetic() { iter_alpha.next().unwrap() } else { c }
}).collect()

// Store closure in variable (problem_1160)
let to_dict = |s: &str| s.chars().fold(HashMap::new(), |mut acc, ch| {
    *acc.entry(ch).or_insert(0) += 1; acc
});
```

---

## 8. Union-Find (DSU)

```rust
// Path compression (problem_3607)
fn find(&mut self, x: i32) -> i32 {
    if self.parent[x as usize] != x {
        self.parent[x as usize] = self.find(self.parent[x as usize]);
    }
    self.parent[x as usize]
}

// Union by rank
fn union(&mut self, x: i32, y: i32) {
    let (rx, ry) = (self.find(x), self.find(y));
    if rx == ry { return; }
    match self.rank[rx as usize].cmp(&self.rank[ry as usize]) {
        Less    => self.parent[rx as usize] = ry,
        Greater => self.parent[ry as usize] = rx,
        Equal   => { self.parent[ry as usize] = rx; self.rank[rx as usize] += 1; }
    }
}
```

---

## 9. Bitwise Tricks

```rust
// OR-accumulate to get union of all bits (problem_2044)
let max_or = nums.iter().fold(0, |acc, &n| acc | n);

// Check parity
x % 2 == 0    // even
x & 1 == 0   // even (faster)

// Divide by 2
x >> 1
```

---

## 10. Numeric Methods

```rust
// Absolute difference without underflow (problem_1455)
a.abs_diff(b)   // works for unsigned types too

// Saturating / checked arithmetic
a.saturating_sub(b)    // clamps at 0 instead of underflowing
a.checked_add(b)       // returns Option<T>

// Integer square root via float (problem_3926)
let sqrt = (n as f64).sqrt() as i32;

// Power
2_u32.pow(level as u32)

// Max/min inline
ret = ret.max(current_value);
a.max(b).max(c)   // three-way max
```

---

## 11. `matches!` Macro

```rust
// Cleaner than long || chains in if conditions (problem_3606)
matches!(category, "electronics" | "grocery" | "clothing")

// Also works in filter
.filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
```

---

## 12. Struct Update Syntax

```rust
// Fill remaining fields with Default values (problem_146)
Node { val: 42, ..Default::default() }
```

---

## 13. Data Structure Patterns

### LRU Cache — `HashMap` + doubly-linked list via `Rc<RefCell<>>`
- HashMap maps key → `Rc<RefCell<Node>>`
- DLL maintains access order; `Weak` for `prev` to avoid cycles
- Move-to-front on access, evict from back on capacity

### Max Stack (problem_716)
- `index_to_number: BTreeMap<usize, i32>` — stack with stable indices
- `num_to_indexes: BTreeMap<i32, Vec<usize>>` — sorted map to find max
- Pop: remove last index from both maps; peekMax: last key of second map

### Range Module (problem_715)
- `left_to_right: BTreeMap<i32, i32>` and `right_to_left: BTreeMap<i32, i32>`
- Bidirectional lookup: given a point, find the interval containing it using `.range().next_back()`

### All-O'one (problem_432)
- `string_frequency: HashMap<String, i32>` — current freq per string
- `frequency_count: BTreeMap<i32, usize>` — how many strings at each freq
- `frequency_string: HashMap<i32, HashSet<String>>` — strings per freq
- O(1) getMinKey/getMaxKey via BTreeMap first/last

### Skip List (problem_1206)
- Levels of linked lists; each node has `right` (same level) and `below` (level down)
- Use `Weak` for `prev` pointers within each level
- Random promotion: `rand::random::<bool>()` per level up

---

## 14. Type Conversion Cheatsheet

```rust
// Widen before arithmetic to avoid overflow
let a: Vec<i64> = a.into_iter().map(|x| x as i64).collect();

// char → u8 for arithmetic
'z' as u8 - 'a' as u8   // = 25

// usize for indexing
some_char as usize

// String → number
"42".parse::<i32>().unwrap()
"42".parse::<i32>()?        // with error propagation

// &str slice → owned String
&s[..3].to_string()
String::from(&s[..3])
```

---

## 15. Collecting with Type Annotation Shorthand

```rust
// Use Vec<_> to let compiler infer element type
.collect::<Vec<_>>()

// HashSet
.collect::<HashSet<_>>()

// String from chars
.collect::<String>()

// Turbofish on collect
let v: Vec<i32> = iter.collect();   // annotation on binding
let v = iter.collect::<Vec<i32>>(); // turbofish inline
```

---

## 16. Miscellaneous

```rust
// Check duplicate chars via set size (problem_953)
s.chars().collect::<HashSet<_>>().len() < s.len()

// Rev iter for palindrome check (problem_1332)
let mut rev = s.chars().rev();
s.chars().all(|c| c == rev.next().unwrap())

// Char is in [a-z] or [A-Z]
c.is_ascii_alphabetic()
c.is_ascii_alphanumeric()
c.is_ascii_digit()

// String find returns Option<usize> (byte index)
s.find('x')

// .position() on iterator (returns Option<usize>)
iter.position(|&x| x == target)
```
