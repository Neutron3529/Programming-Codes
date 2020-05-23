use rayon::prelude::*;
fn main(){
let chunks: Vec<_> = [1, 2, 3, 4, 5].par_chunks(2).collect();
println!("{:?}",&*chunks);
let chunks: Vec<_> = [1, 2, 3, 4, 5].par_chunks(2).with_max_len(1).collect();
println!("{:?}",&*chunks);
let chunks: Vec<_> = [1, 2, 3, 4, 5].par_chunks(2).map(|x|println!("{}",x[0])).collect();
println!("{:?}",&*chunks);
let chunks: Vec<_> = [1, 2, 3, 4, 5].par_chunks(2).with_max_len(1).map(|x|println!("{}",x[0])).collect();
println!("{:?}",&*chunks);
}
