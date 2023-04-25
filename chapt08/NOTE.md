# Tests and Documentation
Normally, the test harness only shows the output of tests that failed. To show the output from tests that pass too, run **cargo test -- --<span style="color: red;">nocapture</span>**.[^1]

# Versions
If you maintain a crate that's at version *1.7* and you decide to remove a function or make any other change that isn't fully backward compatible, you must bump your version number to *2.0*. If you were to call it 1.8, you'd be claiming that the new version is compatible with 1.7, and your users might find themselves with broken builds

# More Nice Things
- When you publish an open source crate on crates.io, your documentation is
automatically rendered and hosted on docs.rs thanks to Onur Aslan.

- If your project is on GitHub, Travis CI can build and test your code on every
push. It’s surprisingly easy to set up; see travis-ci.org for details. If you’re already
familiar with Travis, this .travis.yml file will get you started:
    **language:** rust
    **rust:**
        \- stable
- You can generate a README.md file from your crate’s top-level doc-comment. This feature is offered as a third-party Cargo plug-in by Livio Ribeiro. Run **cargo install cargo-readme** to install the plug-in, then **cargo readme --help** to
learn how to use it.


[^1]: page 196 | Chapter8:Crates and Modules


