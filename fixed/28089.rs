#![feature(plugin)]
#![plugin(docopt_macros)]

docopt!(Args, "
Usage: foo (--help | --version)

Options:
    -h, --help     Show this message
    --version      Show the version of foo
");
