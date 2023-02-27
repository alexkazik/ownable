[![Dependency status](https://deps.rs/repo/github/alexkazik/ownable/status.svg)](https://deps.rs/repo/github/alexkazik/ownable)
[![crates.io](https://img.shields.io/crates/v/ownable.svg)](https://crates.io/crates/ownable)
[![Downloads](https://img.shields.io/crates/d/ownable.svg)](https://crates.io/crates/ownable)
[![Github stars](https://img.shields.io/github/stars/alexkazik/ownable.svg?logo=github)](https://github.com/alexkazik/ownable/stargazers)
[![License](https://img.shields.io/crates/l/ownable.svg)](#license)

# crate ownable

<!-- cargo-rdme start -->

## Derive macro for structs/enums with Cow

To automatically convert `Type<'a>` to `Type<'static>` and more.

## Example

This can be very helpful for types which use borrow with serde.

```rust
#[derive(IntoOwned, ToBorrowed, ToOwned)]
// #[derive(serde::Serialize, serde::Deserialize)]
pub struct Type<'a> {
  // #[serde(borrow)]
  cow: Cow<'a, str>,
  #[ownable(clone)] owned: String, // always clone this field
}
```

Will derive something functionally similar to:
```rust
impl Type<'_> {
  /// Copy the structure and clone the original values if it's not owned.
  /// This is always a deep copy of the structure.
  pub fn into_owned(self) -> Type<'static> {
    Type {
      cow: Cow::Owned(Cow::into_owned(self.cow)),
      owned: self.owned.clone(), // always cloned, as requested
    }
  }
  /// Copy the structure and clone the original values.
  /// This is always a deep copy.
  pub fn to_owned(&self) -> Type<'static> {
    Type {
      cow: Cow::Owned(str::to_owned(self.cow.borrow())),
      owned: self.owned.clone(), // always cloned, as requested
    }
  }
  /// Copy the structure and reference the original values.
  /// This is always a deep copy of the structure.
  pub fn to_borrowed(&self) -> Type {
    Type {
      cow: Cow::Borrowed(self.cow.borrow()),
      owned: self.owned.clone(), // always cloned, as requested
    }
  }
}
```

But actually each function only calls a function of traits, which are derived.

If the derive does not work it can be implemented by hand and still derived for types which use it.

## Possible Errors

 If the following error occurs then one of the fields has a missing trait.
```text
error[E0277]: the trait bound `String: IntoOwned` is not satisfied
```

This can sometimes be fixed with `#[ownable(clone)]` as seen in the example above,
otherwise `AsCopy`/`AsClone` can help.

And as the last resort the impl for the surrounding structure can be hand written.

## AsCopy/AsClone

If the impls for the copy types are not enough or `#[ownable(clone)]` does not work in that
position then `AsCopy` and `AsClone` can be used to wrap a value which then
works in this environment as expected. Both are transparent and do use only exact the same
space as the original type and all impls (Eq, Display, Hash, ...) only pass the calls though
to the inner type

### Example

Example of an more complex type:
```rust
#[derive(IntoOwned, ToBorrowed, ToOwned)]
pub struct Type<'a> {
  cow: Cow<'a, str>,
  nested: Option<Box<Type<'a>>>,
  map: HashMap<AsClone<String>, Cow<'a, str>>,
  #[ownable(clone)] owned: String, // always clone this field
  number: usize, // many copy types have a trait impl, and thus can be used without the `#[ownable(clone)]`
}
```

## Features
* `std` - Traits are also implemented for types which are not in core or alloc (currently `HashMap` and `HashSet`).

`std` is enabled by default.

### Usage
With defaults (includes `std`):
```toml
[dependencies]
ownable = "0.5"
```

With `no_std` (but still requires alloc):
```toml
[dependencies]
ownable = { version = "0.5", default-features = false }
```

<!-- cargo-rdme end -->
## License

This project is licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](https://github.com/alexkazik/ownable/blob/main/LICENSE-MIT))
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](https://github.com/alexkazik/ownable/blob/main/LICENSE-APACHE))

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
