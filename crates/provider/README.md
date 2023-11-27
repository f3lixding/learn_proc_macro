# Provider
There are a couple of things to try out here. 
- [x]: Random function like macros to understand how to work with macro stream.
- [x]: Random function like macros to understand how to do the same but with the help of syn and quote crate. 
- [ ]: Experiment to see how to achieve a "module" like configuration for a binary, similar to that of nginx.

The first two points don't really need any explanations, but the third point is a bit different (and also putting it in writing helps me think).

## NGINX like set up
In nginx, modules are provided as src and included in the conf file for that instance of nginx. 
Configurations for these modules are then provided in the same conf file (in the final, flattened version). 

If we distill the essence of this capability, the configuration capability can be summarized as follows:
1. Dynamic inclusion of modules as dependencies (if we use cargo then that would mean dynamically changing Cargo.toml).
2. Dynamic inclusion of modules in logic for static dispatch.

There isn't a native way to do this. We'll have to rely on scripts external to the binary to change the source code of the binary based on what is supplied to it.
In other words, we are going to have to make it so that the final binary is _supplied_ to configuration as a _build system_. 
Maybe there will be more approaches to implement this in the future, but for now, there is only one that I can think of.

### Custom pre-build followed by post-build logic
In this implementation, point 1 is achieved via a custom pre-build script that seeks out `Cargo.toml` and appends dependencies in the `dependencies` section. 
The challenge here is obtaining said depdencies or how it is resolved. AFAICT there are the following scenarios:
- Dependencies are published on some remote repo like github or crates.io, in which case no special logic is needed here. Cargo should be able to resolve everything for us.
- Dependencies are not published on a remote / build process does not allow network call during build, in which case we would need to rely on other means to resolve the dependency path.  

For this experiment, we shall assume that we don't have an issue with this step (either local crate in the workspace or remote on crates.io).

Point 2 is achieved via a `build.rs` file that reads a configuration file, as supplied by the consumer of the tool. It does the following in the order specified:
- Parses the config 
- Performs any text manipulation (if necessary)
- Seeks out a certain part of the source file and replaces it with the modified text

The above step should be enough to (at build time) dynamically add any dependencies for static dispatch. 
This is where the macro comes in. We can either use the macro at build.rs to replace an entire file, or we can use it to generate a snippet that calls a macro that would in turn expand into code for static dispatch.
