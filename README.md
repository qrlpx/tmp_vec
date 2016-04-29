Problem:

```rust
pub struct Foo {
    ...
}

impl Foo {
    fn do_something(&mut self, ...){
        //FIXME: avoid allocation. can't fix this because T is lifetime-bound.
        let mut guards: Vec<Guard<'x>> = Vec::with_capacity(xxx.len());
        ...
    }
}
```

Solution:

```rust
use tmp_vec::TmpVec;

pub struct Foo {
    tmp_guards: TmpVec<Guard<'static>>,
    ...
}

impl Foo {
    fn do_something(&mut self, ...){
         let mut guards = self.tmp_guards.borrow_mut();
         ...
    }
}
        
```
