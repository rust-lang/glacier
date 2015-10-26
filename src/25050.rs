pub trait NBTTypeHelper where Self: NBTType {
    type Tagtype = Self;
}

pub trait NBTType {
}

impl NBTTypeHelper for NBTType {
    type Tagtype = <Self as NBTTypeHelper>::Tagtype;
}

fn main() {}
