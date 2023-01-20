fn main() {
    let m = "this cat this bat this rat";
    let mut w:Vec<&str> = m.split(' ').collect();
    for i in 0..w.len() {
        if i != w.len(){
        for j in 0..w.len() {
            if j >= w.len(){
                break;
            }
            if i != j{
            if w[i] == w[j]{
                w.remove(j);
                
            }
        }
        }
    }else {
        break;
    }
        }
    println!("unique word:{}",w.len());
}