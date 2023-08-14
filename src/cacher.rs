use std::collections::HashMap;

struct Cacher<E, T>
    where
        T: Fn(&E) -> &str,
{
    query: T,
    value: HashMap<&'static str, &'static str>,
}

impl<E: Eq + std::hash::Hash, T> Cacher<E, T>
    where
        T: Fn(&E) -> &str,
{
    fn new(query: T) -> Cacher<E, T> {
        Cacher {
            query,
            value: HashMap::new(),
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: E) -> &'static str {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.query)(&arg).into();
                self.value.insert(&arg, v);
                v
            }
        }
    }
}