fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    //let t = s;
    //let u = s; // error
    let t = s.clone();
    let u = s.clone();

    struct Person { name: String, birth: i32  }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(),
    birth: 1525 });

    {
        #[derive(Copy, Clone)]
        struct Label { number: u32 }

        fn print(l: Label) {
            println!("STAMP: {}", l.number);
        }

        let l = Label { number: 3 };
        print(l);
        println!("My label number is: {}", l.number);

        //error
        /*
        #[derive(Copy, Clone)]
        struct StringLabel { name: String } // this field does not implement `Copy`
        */
    }


}
