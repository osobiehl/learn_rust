fn main() {
    let v = vec!(1,2,3,4,5);
    for val in &v{
        println!("value is: {}", val);
        

    }
    println!("value inside of vector is: {:?}", v);
    println!("Hello, world!");

    let b = "bitch lasagna";
    let a= "hello";
    {
        // let b = "bitch lasagna"; will not compile
        // let c = lifetime_shit(a, b);
    }
    let c = lifetime_shit(a, b);
    println!("c is: {} ", c);

    // lifetime checking
}

fn lifetime_shit<'a,>(a: &'a str, b: &'a str) -> &'a str{
    if a.len() > b.len() {
        return a;
    }
    else{ return b;}
}
