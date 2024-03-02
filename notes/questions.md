
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
