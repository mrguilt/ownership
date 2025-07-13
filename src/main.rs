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

    println!("\n##Variables and Data Interacting with Move");
    //let st2=st; //Not mutable
    let mut st2=st;
    println!("st2: {}",st2);
    st=String::from("Greetngs");
    println!("Changed the value of st");
    println!("\tst: {}",st);
    println!("\tst2: {}",st2);

    st2=String::from("Salutations");
    println!("Changed the value of st2");
    println!("\tst: {}",st);
    println!("\tst2: {}",st2);
    
}
