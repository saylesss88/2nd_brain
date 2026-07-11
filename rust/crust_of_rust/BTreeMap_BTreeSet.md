# BTreeMap / BTreeSet

Both are ordered collections backed by a B-tree (a self-balancing tree
structure, not a hash table). The core property: **keys are always kept in
sorted order**, and you get that ordering "for free" whenever you iterate.

`BTreeMap<K,V>`:

Same conceptual shape as a `HashMap<K, V>` key/value pairs, one per key, but
internally it's a tree of nodes, each holding several sorted keys (typically
B=6, so nodes hold up to 11 keys before splitting). Requires `K: Ord`, not
`Hash`.

```rs
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(5, "five");
map.insert(1, "one");
map.insert(3, "three");

for (k, v) in &map {
  println!("{k}: {v}"); // prints in order: 1, 3, 5
}
```

**Output**:

```text
1: one
3: three
5: five
```

Key operations HashMap doesn't have:

- `range(1..=3)` — get an iterator over a key range, in order
- `first_key_value()` / `last_key_value(`) — O(log n) min/max
- `pop_first()` / `pop_last()` — remove and return min/max
- Cursor API (`lower_bound`, `upper_bound` on nightly-ish/stable-recent) for
  finding nearest keys

  `BTreeSet<T>`:

Same relationship to BTreeMap as HashSet is to HashMap, it's literally a
`BTreeMap<T, ()>` under the hood. Sorted, unique elements, same range/first/last
operations.

```rs
use std::collections::BTreeSet;

let mut set: BTreeSet<i32> = BTreeSet::new();
set.extend([5, 1, 3, 1]);
// {1, 3, 5} — dedup + sorted automatically

let sub: Vec<_> = set.range(2..5).collect(); // [3]
```

## When to reach for BTreeMap/BTreeSet over Hash-

1. You need sorted iteration. If you're going to sort the output of a HashMap
   every time anyway, just use a BTreeMap and skip the sort.

2. You need range queries. "Give me everything between key A and key B" —
   HashMap can't do this without a full scan. BTreeMap does it in O(log n + k).

3. You need deterministic ordering. HashMap's iteration order is randomized
   per-process (SipHash with a random seed, as a DoS mitigation). If you need
   reproducible output — snapshot testing, deterministic serialization, diffing
   two runs — BTreeMap gives you that automatically.

4. You need min/max repeatedly. first_key_value()/last_key_value() in O(log n)
   beats scanning a HashMap in O(n) every time.

5. Your key type doesn't implement Hash well, but does implement Ord. Some types
   are awkward to hash meaningfully (floats, notably — no Hash impl on f64 due
   to NaN weirdness) but have a clear ordering. BTreeMap<OrderedFloat<f64>, _>
   or similar patterns come up here, though you'll usually still need a wrapper
   type for total ordering.

6. Memory/cache behavior at small-to-medium sizes. B-trees store multiple keys
   per node contiguously, which is cache-friendly. For small maps, BTreeMap can
   sometimes beat HashMap in practice despite the "worse" big-O, because there's
   no hashing overhead and better locality. This isn't a strong reason to choose
   it, but it's not a downside either.
