fn main(){
    let mut ch = '@';
    let ch_ptr = &mut ch; 
    *ch_ptr = '.';  

    println!("{ch}"); 
}