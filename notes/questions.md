
# 2024-03-02 Sat

## How do we put tests as a module in the same file?

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

from [The Rust Book, Chapter 11](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)



# 2024-03-04 Sun


## Should we remove Box2D `x1()`, `y1()`, `x2()`, `y2()`?

Yes: they only save a character each compared to `p1.x`, `p1.y`, `p2.x`, `p2.y`, and their meaning is less obvious than `p1.x`, etc.

No: removing them will break a lot of files.
Counterpoint: the breakages will be obvious because of compiler errors.

Decision?


## Should we remove Box2D `width()`, `height()`?

Yes: as above, they're less obvious than `size().width`, `size().height`.

No: unlike p1.x, they save a lot more characters, e.g. `width()` is 7 characters shorter than `size().width()`.

Decision?


## Keep Box2D `size()`?

Probably yes. More obvious than `box2d.p2 - box2d.p1`, and probably shorter in most cases.
