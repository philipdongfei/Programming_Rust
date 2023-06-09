# Structs
## Named-Field Structs
In a struct expression, if the named fields are followed by **.. EXPR**, then any fields not mentioned take their values from **EXPR**, which must be another value of the same struct type.


    // Receive the input Broom by value, taking ownership.
    fn chop(b: Broom) -> (Broom, Broom) {
        // Initialize `broom` mostly from `b`, changing only `height`. Since
        // `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
        let mut broom1 = Broom {height: b.height / 2, .. b }; // if the named fields are followed by `.. EXPR`, then any fields not mentioned take their values from EXPR, which must be another value of the same struct type.
    
        // Initialize `broom2` mostly from `broom`. Since `String` is not
        // `Copy`, we must clone `name` explicitly.
        let mut broom2 = Broom { name: broom1.name.clone(), .. broom1  }; // ".. broom1" -> "**.. EXPR**"
    
        // Give each fragment a distinct name.
        broom1.name.push_str(" I");
        broom2.name.push_str(" II");
    
        (broom1, broom2)
    }



## Passing Self as a Box, Rc, or Arc
- If it can pass ownership of the *Rc*, it simply hands over the pointer.

        let shared_node = Rc::new(Node::new("first"));
        shared_node.append_to(&mut parent);

- If it needs to retain ownership of an *Rc*, it just bumps the reference count. 

        shared_node.clone().append_to(&mut parent);

- Only if it owns the *Node* itself must it call *Rc::new* to allocate heap space and move the *Node* into it.Since *parent* will insist on referring to its children via *Rc<Node>* pointers, this was going to be necessary eventually. 

        let owned = Node::new("owned directly");
        Rc::new(owned).append_to(&mut parent);

## Structs with Lifetime Parameters
Here's a function to scan a slice and return an *Extrema* value whose fields refer to its elements:

    fn find_extrema</'s>(slice: &'s [i32]) -> Extrema</'s> {
            //TODO: find extrema 
    }



Because it's so common for the return type to use the same lifetime as an argument, Rust lets us ommit the lifetimes when there's one obvious candidate.

    fn find_extrema(slice: &[i32]) -> Extrema {
    
    }


## Interior Mutability
Cells are easy to use. The other drawback is less obvious and more serious: cells--and any types that contain them--are not thread-safe. Rust therefore will not allow multiple threads to access them at once.

