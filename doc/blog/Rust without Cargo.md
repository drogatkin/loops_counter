# Rust without Cargo
## Why?
Most people can't imagine programming in Rust without the Cargo utility. It is a very powerful tool takes care of all tasks required to build packages and finally run a Rust program. 
It uses a special manifest file TOML where a developer provides a required information about the project (or package) itself and all required external dependencies.
There is a huge public source **https://crates.io/** of open source solutions in Rust which can be used in a Rust program just specifying a crate name and version  in Cargo manifest file. 
 However most of these solutions have a questionable quality (many of them just a dumb port from C code with lot unsafe inserts) and you use them on own risk. Rust is
 a machine code compilable language and it requires that all linked code was available in a source code, because it's hardly possible to provide a precompiled version for all
target platforms. So Cargo takes care of loading and compiling a source code in object files for the target platform. 
 Any crate code can depend on other crates which can be depended on other crates and so on. So when you add just one dependency in TOML file,
 Cargo can be pushed to download and compile hundred of crates. Several crates for a project can depend on the same crates, so Cargo has to build
 a summarized dependency tree and then to decide about a right order of compilations of all crates. Crates can be taken not only from crates.io, they can be taken from
an arbitrary git repository, however it should follow a certain directories structure that Cargo can issue right compilation instructions.


Here is a simple example of TOML file:
```
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
time = "0.1.12"
regex = "0.1.41"
```

Using a version control repository for a dependency:
```
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```
As you can see, Cargo uses declarative approach, when you tell just what you need and do not care how Cargo will deal with that.
Although I am a big fan of declarative approach, I think that imperative is more flexible for build tasks.
However, does Cargo do so complicated work? If you do not have many dependencies from crates.io, or a deploying your Rust project
can be different than Cargo supports, then you can easily drop using Cargo. This article addresses all required steps for 
successful build of dependencies and to make the final package.

## How to compile a crate just using rustc
The rustc is a quite smart compiler itself and can compile Rust code in different formats including libraries or a final executable.
All compilation keys can be specified in the command line or provided in a separate file. 

First important key is : --crate-type. When it isn't specified then 
rustc builds an executable which can be ran. The --crate-type=lib has to be specified when a crate is built,

The rustc will try to infer the build crate name unless it is specified using: --crate-name. The name should follow the standard Rust identifier naming convention.

The rest is just specifying where result as .rlib file should be generated. A specific location can be provided in the -o option, or just specifying a common
lib (crate) files location using the --out-dir command line option. The result name will be calculated automatically based on the crate name. 
Although using -o option allows to customize a result name, it still has to follow the certain pattern as:

> lib\<crate name\>.rlib

A name of the main Rust file has to be specified in the rustc command line. It is usually lib.rs for a crate, although it can be a name of some Rust file, as
somefile.rs or mod.rs. The rustc will consider the specified file as the main module of the crate and export names specified in it to be able
to resolve them from other Rust crates or code. Since preparing a top module code common when Cargo is used or not, no details will be given here.

## Specifying external dependencies

It is most complicated task Cargo provides, therefore the Cargo job will be uncovered here. The rustc tries to find all unresolved names inside the built package 
in other libraries (crates) upon a fully qualified name. The names looks like:
```
[[scope qualifier]::]crate name::module name::...module name::final name inside the most inner module
```
If a leading :: specified, then a search for a crate's names happened from the root level. If the crate is specified as the root scope, then
a search will happen in the current crate only and no external crates will be used to resolve a name.

A crate resolution algorithm looks like : 
1. search inside the current crate
2. search in crates specified as the extern crate \<name\>[ as \<alias\>];, a specific crate location can be qualified in --extern compilation directive.
4. search in crates specified by -L directory. The extern crate Rust definition isn't required for this case. The use directive can't be used since it's mostly a syntax sugar.

Although the rustc is extremely flexible in a searching names, Cargo limits the capabilities to make them extremely unambiguous and all external crates are specified in 
--extern command line options. Each such directive contains a crate name and it's location. So only one thing has to be assured, that all external crates in dependency
have to be built prior the current crate built. If dependency tree isn't very complex, it is easy to specify it without Cargo. 

## Using generic building utility to build a Rust project

The article demonstrates as RustBee is used to build a Rust project. Any other build tool with similar capabilities can be used too.

Consider the following directory structure: 

```
----|
    |-- crates
             |-- Crate 1
             |-- Crate 2
             |-- ...
    |-- project 1
    |-- project 2
    |--  ....
    
```
The crates directory itself contains .rlib files for every built crate. The crate source directory name has to match the crate name. 

Project directories are placed on the level of the *crates*. A build script has to provide a list of used crates in the order of their dependencies, for example:
```
crates=[time,web_cgi]
```
Every listed crate can depend on earlier listed crates. The time crate has no dependencies, however the web_cgi crate may depend on the time. 
This order assures that any built crate will be built after all its dependency crates built prior.

A portion of the build script to build all dependency crates looks like:

```
dep_crates=[]

target build_crates:. {
    dependency{
    	anynewer(${~cwd~}/../crates/*.rs, # or just */*.rs
    			${~cwd~}/../crates/*.rlib)
    }
	for crate:crates {
		# any dependent after rebuild is required, has to be rebuilt too
	    if {
	        anynewer(${~cwd~}/../crates/${crate}/*.rs,${~cwd~}/../crates/lib${crate}.rlib)
	       then {
	       	   assign(bild_follow, true)
	       }
	    }
	    if {
	    	eq(bild_follow,true)
	    then {
		    display(Building crate ${crate})
			exec rustc::  (
	    	  --color, always,
	    	 -C, opt-level=2,
	    	 -L, ../crates,
	    	 --crate-type=lib,
	    	 --edition, 2021,
	    	 dep_crates,
	    	 --crate-name, crate,
	        --out-dir, ../crates,
	         ../crates/${crate}/lib.rs
	       )
	       if {
		         neq(${~~}, 0)
		         then {
		            panic("${crate} compilation error(s)")
		         }
    	   }
       } # end then
       } # end if
       array(dep_crates,--extern,${crate}=../crates/lib${crate}.rlib)
       assign(dep_crates,~~)
	}
}
``` 
First, the script just detects if any crate's source file modified after a crate binary file (.rlib).  
If such condition has been found, then it loops across all crates to find a first crate which
needs to be rebuilt. All crates which depends on the crate, have to be rebuilt too. The script acknowledges the condition.

The rustc checks also the condition and if any crate in dependencies of some crate was updated, 
it refuses the build and request to check the dependency.

The script provides the standard location for every built crate and also builds a list of --extern crates for every
crate built. 

Such build logic is quite reliable to any source code of crates change. However in a case of any discrepancy happens,
there is a cleaning of all crates generated directory build target listed  below:
```
target clean_crates:. {
     dependency {true}
	for crate:crates {
	   display(Cleaning ../crates/lib${crate}.rlib)
		exec rm  (
	        ../crates/lib${crate}.rlib
	    )
	}
}
```
## A building Rust project with external dependencies

Any other Rust project can use the built crates.

The --extern compilation directives can be omitted for the final package build since all dependencies are
already in the standard place under the standard name. For example:
```
crate_dir=../../rusthub/src/rust/crates

target build:. {
   dependency {
       anynewer(bee.7b,${cgi_dir}/${project})
   }
   dependency {
         anynewer(${~cwd~}/*.rs,${~cwd~}/${cgi_dir}/${project})
   }
   display(Compiling ${main} ...)
   exec rustc::  (
       --color, always,
       -L,all=${crate_dir},
       -o,
       ${cgi_dir}/${project},
       ${main}.rs
   )
     if {
         neq(${~~}, 0)
         then {
            panic("compilation error(s)")
         }
     }
}
```
However you  still need to provide the extern directives in the Rust code like below:
```
extern crate time;
extern crate web_cgi as web;
.....
use std::fs::read_to_string;
.....
```
As you can see, a building Rust project without Cargo is a quite possible and is not really complicated.

The script code can be accommodated using any other directories structure. It's another benefit to
do not use Cargo.

RustBee scripting build tool is used in the article, however any other tool including just shell scripts can be used
 for an automation of a building procedure.

Happy Rusting!

## References

1. The article was inspired by [the notes](https://gitlab.com/tools6772135/rusthub/-/blob/master/doc/rust/README.md)
2. RustBee scripting [tool](https://gitlab.com/tools6772135/rusthub/-/blob/master/doc/rustbee/README.md?ref_type=heads)
