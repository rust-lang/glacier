trait BufMut {}
struct Bytes;
struct BytesMut;

pub trait Future {
    type Item;
}

pub trait Service {
    type Response;
    type Future: Future<Item = Self::Response>;
}

pub trait ThriftService<F>:
    Service<
        Response = FramingEncoded<F>,
        Future = Box<Future<Item = FramingEncodedFinal<F>>>,
    >
where
    F: Framing,
{
    fn get_service(
        &self,
    ) -> &Service<
        Response = Self::Response,
        Future = Self::Future,
    >;
}

pub trait BufMutExt: BufMut {
    type Final;
}

impl BufMutExt for BytesMut {
    type Final = Bytes;
}

pub type FramingEncoded<F> = <F as Framing>::EncBuf;
pub type FramingEncodedFinal<F> = <<F as Framing>::EncBuf as BufMutExt>::Final;

pub trait Framing {
    type EncBuf: BufMut;
}

pub struct SRHeaderTransport;
impl Framing for SRHeaderTransport {
    type EncBuf = BytesMut;
}

pub type BoxService<H> = Box<
    ThriftService<
            SRHeaderTransport,
            Response = Bytes,
            Future = Box<Future<Item = Bytes>>,
        >,
>;

fn with_factory<H>(self, factory: BoxService<H>) {}

fn main() {}
