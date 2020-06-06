#[test]
fn cache_works() {
    use vial::TypeCache;
    let cache = TypeCache::new();

    let v = "Hiya".to_string();
    cache.set(v.clone());
    assert_eq!(&v, cache.get::<String>().unwrap());
    assert_eq!(&v, cache.get::<String>().unwrap());

    let v2 = "Something Else".to_string();
    cache.set(v2.clone());
    assert_eq!(&v2, cache.get::<String>().unwrap());
    assert_eq!(&v2, cache.get::<String>().unwrap());

    assert_eq!(None, cache.get::<&str>());
    assert_eq!(None, cache.get::<usize>());

    cache.set(314);
    assert_eq!(None, cache.get::<usize>());
    cache.set::<usize>(314);
    assert_eq!(Some(&314), cache.get::<usize>());
}

#[test]
fn cache_works_with_structs() {
    use vial::TypeCache;
    let cache = TypeCache::new();

    #[derive(Clone, Debug, PartialEq)]
    struct MyConfig {
        host: String,
        port: usize,
    }

    let config = MyConfig {
        host: "localhost".into(),
        port: 7654,
    };

    assert_eq!(None, cache.get::<MyConfig>());
    cache.set(config.clone());
    assert_eq!(&config, cache.get::<MyConfig>().unwrap());
}
