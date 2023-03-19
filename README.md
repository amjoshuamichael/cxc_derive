# 🪞 cxc-derive: Make any type usable in cxc

```rust
use cxc::Unit;
use cxc::XcReflect;

#[derive(Default, Clone, Copy, Debug, XcReflect, PartialEq, Eq)]
pub struct Numbers5 {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub e: i32,
}

#[test]
fn numbers_5() {
    let mut unit = Unit::new();

    unit.add_reflect_type::<Numbers5>();
    unit.add_external_default::<Numbers5>();

    unit.push_script(
        "
        some_numbers(); Numbers5 {
            ; Numbers5 { a = 4, b = 9, c = 39, ++ }
        }
        "
    ).unwrap();

    let some_numbers = unit.get_fn("some_numbers").unwrap().downcast::<(), Numbers5>();
    let numbers5 = some_numbers();

    assert_eq!(numbers5, Numbers5 { a: 4, b: 9, c: 39, ..Default::default() });
}
```

Supports regular struct types, enums, tuples, function types, references, raw pointers, etc. AFAIK, the only thing it doesn't support it Unions, because those aren't in [cxc](https://github.com/amjoshuamichael/cxc). If there's anything that it doesn't support that you think ti should, then please submit an [issue](https://github.com/amjoshuamichael/cxc_derive/issues/new/choose)
