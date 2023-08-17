# cargo-public-api-benchmarks
Benchmarks for cargo-public-api and related crates.

Currently experimental.

## Low-hanging fruits

Here is a list of things that probably would significantly improve the performance of the `public-api` crate:

* Make `UnprocessedItem::parent_path` contain `Vec<Rc<PathComponent>>` instead of `Vec<PathComponent>`. That should give a noticeble performance boost since `PathComponent` is cloned a lot, and wrapping it inside of an `Rc` should significantly reduce the number of heap allocations.
* Use multiple threads to process `ItemProcessor::work_queue` (and use `Arc` instead of `Rc` above)
* Instead of rendering `impl`s to group them in the output (which according to my profiling takes a long time), use something like a [`DisambiguatedDefPathData`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/definitions/struct.DisambiguatedDefPathData.html) with a `disambiguator: u32`.
