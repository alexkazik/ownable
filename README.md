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

## Generics

The derive macro supports all kinds of generics: lifetimes, types, consts. And the first
two with bounds and all as many times as you want.

## References

References are not supported out of the box, because they can't be changed into an owned type.

But it's possible to specify which lifetime(s) are used solely for references and then those
will be always copied (the reference) and thus the lifetime is not changed.

Please note that not only the type containing a reference but also types containing such a
type are required to be marked.

### Example

```rust
#[derive(IntoOwned, ToBorrowed, ToOwned)]
#[ownable(reference = "'b")]
pub struct Inner<'a, 'b> {
  cow: Cow<'a, str>,
  referenced: &'b str,
}

// Also types, containing types with references, must be marked.
#[derive(IntoOwned, ToBorrowed, ToOwned)]
#[ownable(reference = "'b")]
pub struct Outer<'a, 'b> {
  inner: Inner<'a, 'b>,
}
```
Will derive functions with these signatures:
```rust
impl<'b> Inner<'_, 'b> {
    pub fn into_owned(self) -> Inner<'static, 'b> {
        // Call the trait, which is also derived
    }
    pub fn to_owned(&self) -> Inner<'static, 'b> {
        // Call the trait, which is also derived
    }
    pub fn to_borrowed(&self) -> Inner<'_, 'b> {
        // Call the trait, which is also derived
    }
}

// The `Outer` will look similar.
```

## Possible Errors

 If the following error occurs then one of the fields has a missing trait.
```text
error[E0277]: the trait bound `String: IntoOwned` is not satisfied
```

This can sometimes be fixed with `#[ownable(clone)]` as seen in the example above,
otherwise `AsCopy`/`AsClone` can help.

And as the last resort the impl for the surrounding structure can be hand written.

## Attributes

### clone

With `#[ownable(clone)]` and `#[ownable(clone = false|true)]` it's possible to denote that this
enum|struct/variant/field should always be cloned. It can be overwritten (i.e. set to true at
top level and then false at the fields to not be cloned).

For an example see the at the top.

### function

With `#[ownable(function = false)]` at top level (enum/struct) the functions mentioned above
are not implemented, only the traits.

### reference

With `#[ownable(reference = "..")]` one or more comma separated lifetimes can be supplied to
be used for references only, see [References](#references) above.

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
* `beef` - Traits are also implemented for `beef::Cow` (generic).

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
