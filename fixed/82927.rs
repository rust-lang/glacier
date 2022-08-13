trait SendEqAlias<T> = PartialEq;
struct Foo;
struct Bar<T>(SendEqAlias<T>);
