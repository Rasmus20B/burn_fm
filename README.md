# BurnFM - A FileMaker Utility Library
BurnFM provides facilities to allow developers to interact with the FileMaker FMP12 file format, with the goal of providing an interface to build helpful development tools and addons.

## Quick-Start
To obtain a local copy of BurnFM all you need to do is clone this repository.
```
$ git clone https://github.com/rasmus20b/burn_fm.git
```

and use cargo to build and run.
```
$ cargo run
```

## Design Goals
Due to the proprietary nature of the FMP12 format and the lack of documentation for FileMaker plugin development, it has become necessary to develop external tooling to bring FileMaker's development experience up to modern standards. 

The most important features this project aims to provide are **testing** and **version control** via a new high level language representation, as well as an easy to use library that allows users to build tools to aid them in their FileMaker development.
### Testing
Perhaps the most important feature missing from FileMaker. The BurnFM high-level language provides the `assert()` script step for high-level test scripts. A failed assert step will trigger a test fail. Tests can be defined in *burn* files, and paired with an Fmp12 or Burn file in the command line. For example, running a series of tests defined in a burn file on an Fmp12 file called blank:
``` 
$ burn_fm -d blank.fmp12 -t blank_tests.burn 
```

### Version Control
Currently, FileMaker is able to export a file's schema into a high level representation called a **database design graph** (DDR), in HTML or XML, and allow users to view changes between versions of the same file. There also exist external tools like FMDiff that are able to highlight these changes similar to traditional version control software.

Unfortunately, these high-level representations cannot be 're-compiled' into the Fmp12 format, only providing a 1-way interface. Meaning loading a previous commit is only possible if you also track the Fmp12 files which are in a binary format, leading to bloated storage and unhelpful noise in diffs between commits.

Collaboration is only possible either by developers staying outside of each other's 'areas of modification', or by each developer downloading the file, making changes, and manually converging the differences (possibly aided by the use of tools previously mentioned). Because of the hassle, most FileMaker developers opt for making changes in production.

### *Burn* high-level language
By Providing a text-based language, BurnFM is able to provide the benefits of version control to FileMaker developers, as well as editor features such as syntax highlighting and code completion.

The language is able to define:
- Tables
- Table Occurences
- Relationships
- Scripts
- Tests
- Value Lists
- Layout metadata such as which table occurrence it is connected to.

Scripting in this language is designed to be easy to pick up for those already used to FileMaker's scripting engine, keeping most of the same visuals and functions of that language. For example:
```
script: [
  define basic_script(x, y) {
    if(x > y) {
	  exit_script(x);
	}
    set_variable(i, x);
    loop {
      exit_loop_if(i == y);
      set_variable(i, i + 1);
    }
    exit_script(i);
  }
]
```

## Contributing
Contributions are welcome and greatly appreciated. If you have any suggestions or improvements, please fork the repo and create a pull request. 
