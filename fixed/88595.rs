#![feature(generic_associated_types)]    
#![feature(type_alias_impl_trait)]    
    
fn main() {    
    println!("Hello, world!");    
}    
    
trait A<'a> {    
    type B<'b>: Clone    
    where    
        'a: 'b;    
                                         
    fn a(&'a self) -> Self::B<'a>;       
}    
    
struct C;    
    
impl<'a> A<'a> for C {    
    type B<'b> = impl Clone;               
        
    fn a(&'a self) -> Self::B<'a> {}       
}   
