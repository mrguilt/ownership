fn main() {
    println!("# Understanding Ownership\n");

    let s="Guten Tag"; //string literal
    {
        println!("s in brackets");
        let s="hello";
        println!("{}",s);
    }
    println!("Out of brackets: {}",s);

    println!("\n## The String Data Type\n");
    let mut st=String::from("Hello");
    println!("After Declaration: {}",st);
    st=String::from("Howdy");
    println!("Changed it: {}",st);
    st.push_str(", bucaroo");
    println!("Add to the end: {}",st);
}
