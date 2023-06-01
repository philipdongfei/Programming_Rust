# Closures

Other examples of standard library features that accept closures include:

- **Iterator** methods such as *map* and *filter*, for working with sequential data.
- Threading APIs like **thread::spawn**, which starts a new system thread. Concurrency is all about moving work to other threads, and closures conveniently represent units of work.
- Some methods that conditionally need to compute a default value, like the *or_insert_with* method of *HashMap* entries.

## Capturing Variables

A closure can use data that belongs to an enclosing function.

### Closures That Borrow

    /// Sort by any of several different statistics.
    fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
        cities.sort_by_key(|city| -city.get_statistic(stat));
    }

In this case, when Rust creates the closure, it automatically borrows a reference to *stat*. It stands to reason: the closure refers to *stat*, so it must have a reference to it.
In short, Rust ensures safety by using lifetimes instead of garbage collection.

### Closures That Steal

The solution to both problems is the same: tell Rust to **move** *cities* and *stat* into the closures that use them instead of borrowing references to them.

    use std::thread;
    fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
        -> thread::JoinHandle<Vec<City>>
        {
            let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) }; // key_fn, takes ownership of stat
            thread::spawn(move || {
                cities.sort_by_key(key_fn);
                cities
            })
        }

Note that **||** is the closure's empty argument list.
The only thing we've changed is to add the **move** keyword before each of the two closures. The **move** keyword tells Rust that a closure doesn't borrow the variables it uses: it steals them.

Rust thus offers two ways for closures to get data from enclosing scopes: moves and borrowing.
- Just as everywhere else in the language, if a closure would **move** a value of a copyable type, like i32, it copies the value instead. So if Statistic happened to be a copyable type, we could keep using stat even after creating a **move** closure that uses it.
- Values of noncopyable types, like *Vec<City>*, really are moved: the preceding code transfers *cities* to the new thread, by way of the **move** closure. Rust would not let us access *cities* by name after creating the closure.
- As it happens, this code doesn't need to use *cities* after the point where the closure moves it. If we did, though, the workaround would be easy: we could tell Rust to clone *cities* and store the copy in a different variable. The closure would only steal one of the copies--whichever one it refers to.

We get something important by accepting Rust's strict rules: thread safety.


## Function and Closure Types

You can do all the same things with functions that you do with other values. You can store them in variables. You can use all the usual Rust syntax to compute function values:

    let my_key_fn: fn(&City) -> i64 = 
        if user.prefs.by_population {
            city_population_descending
        } else {
            city_monster_attack_risk_descending
        };
    cities.sort_by_key(my_key_fn);

Structs may have function-typed fields. Generic types like *Vec* can store scads of functions, as long as they all share the same *fn* type. And function values are tiny: a *fn* value is the memory address of the function's machine code, just like a functon pointer in C++.

The second argument causes a type error. To support closures, we must change the type signature of this function. It needs to look like this:

    fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
        where F: Fn(&City) -> bool
        {
            let mut count = 0;
            for city in cities {
                if test_fn(city) {
                    count += 1;
                }
            }
            count
        }

We have changed only the type signature of *count_selected_cities*, not the body. The new version is generic. It takes a *test_fn* of any type F as long as F implements the special trait *Fn(&City) -> bool*. This trait is automatically implemented by all functions and most closures that take a single *&City* as an argument and return a Boolean value:

    fn(&City) -> bool // fn type (functions only)
    Fn(&City) -> bool // Fn trait (both functions and closures)

This special syntax is built into the language. The -> and return type are optional; if omitted, the return type is ().

The new version of *count_selected_cities* accepts either a function or a closure:

    count_selected_cities(
        &my_cities,
        has_monster_attacks
    ); // ok
    
    count_selected_cities(
        &my_cities,
        |city| city.monster_attack_risk > limit
    ); // also ok

Well, a closure is callable, but it's not a fn.

In fact, every closure you write has its own type, because a closure may contain data: values either borrowed or stolen from enclosing scopes. This could be any number of variables, in any combination of types. So every closure has an ad hoc type created by the compiler, large enough to hold that data. No two closures have exactly the same type. But every closure implements an *Fn* trait; 
Since every closure has its own type, code that works with closures usually needs to be generic, like *count_selected_cities*.


## Closure Performance

Rust's closures are designed to be fast: faster than function pointers, fast enough that you can use them even in red-hot, performance-sensitive code.



## Closures and Safety

### Closures That Kill

### FnOnce

The first time you call a **FnOnce** closure, *the closure itself is used up*. It's as though the two traits, **Fn** and **FnOnce**, were defined like this:

    // Pseudocode for `Fn` and `FnOnce` traits with no arguments.
    trait Fn() -> R {
        fn call(&self) -> R;
    }

    trait FnOnce() -> R {
        fn call_once(self) -> R;
    }

### FnMut

There is one more kind of closure, the kind that contains mutable data or mut references.

Rust considers non-mut values safe to share across threads. But it wouldn't be safe to share noon-mut closures that contain mut data: calling such a closure from multiple threads could lead to all sorts of race conditions as multiple threads try to read and write the same data at the same time.

*FnMut* closures are called by *mut* reference, as if they were defined like this:

    trait FnMut() -> R {
        fn call_mut(&mut self) -> R;
    }

Any closure that requires *mut* access to a value, but doesn't drop any values, is an *FnMut* closure.

let's take a step back and summarize what you've learned about the three categories of Rust closures.

- *Fn* is the family of closures and functions that you can call multiple times without restriction. This highest category also includes all *fn* functions.
- *FnMut* is the family of closures that can be called multiple times if the closure itself is declared *mut*.
- *FnOnce* is the family of closures that can be called once, if the caller owns the closure.

Every *Fn* meets the requirements for *FnMut*, and every *FnMut* meets the requirements for *FnOnce*.

Instead, *Fn()* is a subtrait of *FnMut()*, which is a subtrait of *FnOnce()*.


### Copy and Clone for Closures

A non-move closure that doesn't mutate variables holds only shared references, which are both *Clone* and *Copy*, so that closure is both *Clone* and *Copy* as well:
    
    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;  // This closure is `Copy`, so...
    assert_eq!(add_y(copy_of_add_y(22)), 42); // ... we can call both.

On the other hand, a non-move closure that does mutate values has mutable references within its internal representation. Mutable references are neither *Clone* nor *Copy*.

For a *move* closure, the rules are even simpler. If everything a *move* closure captures is *Copy*, it's *Copy*. If everything it captures is *Clone*, it's *Clone*.

    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");

This *.clone()(...)* syntax is a little weird, but it just means that we clone the closure and then call the clone.
When *greeting* is used in *greet*, it's moved into the struct that represents *greet* internally, because it's a *move* closure. So, when we clone *greet*, everything inside it is cloned, too.

## Callbacks

A lot of libraries use *callbacks* as part of their API: functions provided by the user, for the library to call later.

We can begin by declaring a few types to represent HTTP requests and responses:

    struct Request {
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Vec<u8>
    }
    
    struct Response {
        code: u32,
        headers: HashMap<String, String>,
        body: Vec<u8>
    }

we showed a *Salad* type that had the same problem:
    
    struct Salad<V: Vegetable> {
        veggies: Vec<V>
    }

The solution here is the same as for the salad: since we want to support a variety of types, we need to use **boxes** and **trait objects**:

    type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

    struct BasicRouter {
        routes: HashMap<String, BoxedCallback>
    }

Each box can contain a different type of closure, so a single *HashMap* can contain all sorts of callbacks. Note that the type parameter **C** is gone.

    impl BasicRouter {
        // Create an empty router.
        fn new() -> BasicRouter {
            BasicRouter { routes: HashMap::new() }
        }
    
        // Add a route to the router.
        fn add_route<C>(&mut self, url: &str, callback: C) 
        where C: Fn(&Request) -> Response + 'static
        {
            self.routes.insert(url.to_string(), Box::new(callback));
        }
    }

> Note the two bounds on C in the type signature for **add_route**: a particular **Fn** trait and the 'static lifetime. Rust makes us add this 'static bound. Without it, the call to **Box::new(callback)** would be an error, because it's not safe to store a closure if it contains borrowed references to variables that are about to go out of scope.

Finally, our simple router is ready to handle incoming requests:
    
    impl BasicRouter {
        fn handle_request(&self, request: &Request) -> Response {
            match self.routes.get(&request.url) {
                None => not_found_response(),
                Some(callback) => callback(request)
            }
        }
    }

At the cost of some flexibility, we could also write a more space-efficient version of this router that, rather than storing trait objects, uses *function pointers*, or *fn* types.

In fact, closures that don't capture anything from their environment are identical to function pointers, since they don't need to hold any extra information about captured variables.
    
    fn add_ten(x: u32) -> u32 {
        x + 10
    }
    let fn_ptr: fn(u32) -> u32 = add_ten
    let eleven = fn_ptr(1); // 11
    // `closure_ptr` identical to `fn_ptr`.
    let closure_ptr: fn(u32) -> u32 = |x| x + 10;
    let eleven = closure_ptr(1); // 2

Unlike capturing closures, these function pointers take up only a single *usize*.

    struct FnPointerRouter {
        routes: HashMap<String, fn(&Request) -> Response>
    }

    impl FnPointerRouter {
        // Create an empty router.
        fn new() -> FnPointerRouter {
            FnPointerRouter { routes: HashMap::new() }
        }
    
        // Add a route to the router.
        fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response)
        {
            self.routes.insert(url.to_string(), callback);
        }
    }

As laid out in Figure 14-1, closures have unique types because each one captures different variables, so among other things, they're each a different size. If they don't capture anything, though, there's nothing to store. By using *fn* pointers in functions that take callbacks, you can restrict a caller to use only these noncapturing closures, gaining some perfomance and flexibility within the code using callbacks at the cost of flexibility for the users of your API.


## Using Closures Effectively

As we've seen, Rust's closures are different from closures in most other launguages. The biggest difference is that in languages with GC, you can use local variables in a closure without having to thingk about lifetimes or ownership.

For every element of a user interface, an MVC(Model-View-Controller) framework creates three objects: a *model* representing that UI element's state, a *view* that's responsible for its appearance, and a *controller* that handles user interaction.

You can't implement this pattern in Rust without making some changes. Ownership must be made explicit, and reference cycles must be eliminated. The model and the controller can't have direct references to each other.




