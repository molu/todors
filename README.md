## To-do List Application

The capstone project of the "Rust for n00bs" course (https://training.zeropointsecurity.co.uk/courses/rust-for-n00bs). 
The goal is to create a to-do list application using all the knowledge and skills presented throughout this course.

### Usage

To run in debug mode:
```shell
cargo run
```

To build and run an optimized version:
```shell
cargo build
./target/build/todors // or ./target/build/todors.exe
```

```shell
[ Create new todo:                     create ]
[ View all todos:                        list ]
[ Select a todo:                  select <ID> ]
[ Show todo information (select mode):   show ]
[ Update a todo (select mode):         update ]
[ Start progress on todo (select mode): start ]
[ Finish todo (select mode):         complete ]
[ Delete a todo (select mode):         delete ]
[ Display this help message:             help ]
[ Exit the program:                      exit ]
```