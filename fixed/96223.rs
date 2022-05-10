use std::marker::PhantomData;

pub trait Deserialize<'de>: Sized {
    // fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    // where
    //     D: Deserializer<'de>;
}

pub trait Yokeable<'a>: 'static {
    type Output: 'a;
}

pub trait DataMarker {
    type Yokeable: for<'a> Yokeable<'a>;
}

pub struct YokeTraitHack<T>(pub T);

impl<'de, T> Deserialize<'de> for YokeTraitHack<T>
where
    T: Deserialize<'de> {}

struct FsDataProvider;

impl<M> DynProvider<M> for FsDataProvider
where
    M: DataMarker,
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de> {

}
// pub struct DataPayload<M: DataMarker>(<M::Yokeable as Yokeable<'static>>::Output);

pub trait DynProvider<M> 
where
    M: DataMarker, 
{
    // fn load_payload(
    //     &self,
    //     key: &str
    // ) -> DataPayload<M>;
}


pub struct CodePointSet<'a>(&'a [u8]);

impl<'a> Yokeable<'a> for CodePointSet<'static> {
    type Output = CodePointSet<'a>;
}

pub struct CodePointSetMarker;

impl DataMarker for CodePointSetMarker {
    type Yokeable = CodePointSet<'static>;
}
fn icey_bounds<D: DynProvider<CodePointSetMarker>>(p: &D) {
    // let ascii_hex_digit = get_ascii_hex_digit(p).expect("hi");
}

fn trigger_ice() {
    let p = FsDataProvider;
    icey_bounds(&p);
}

fn main() {}
