# router

Router. The ultimate route finder.

**IMPORTANT**: If you want to load large map files, it is highly recommended that you use the release build because the debug build is **very** slow.

## Buidling
### Release
```sh
  cargo build --release
```

### Debug
```sh
  cargo build
```

## Running (On Linux)
```sh
# For the release build please run:
target/release/router

# For the debug build please run:
target/debug/router
```

## Usage

Please use *'--help'* to list all available commands.<br><br>
You can use *'-graph \<file\>'* to load a graph file in the format described [here](https://fmi.uni-stuttgart.de/alg/research/stuff/ "FMI Uni Stuttgart"). This will also create afterwards the nearest data structure and is required first before running a query.<br>
Using *'-lon \<longitude\>'* and *'-lat \<latitude\>'* you can specify the longitude and latitude respectively as a floating point number. By using the flag *'--naive'* you can also let **router** search the nearest node using a numb, naive way to later compare the results. This location is also searched via the nearest data structure (a QuadTree).<br>
You can also load a file with queries using *'-que \<file\>'*. This file should contain, line by line each query in the following format:<br><br>
**\<source node id\> \<target node id\>**<br>
seperated by a space in the middle.
<br><br>
The queries will be ran multi-threaded using 4 threads or less, depending on the system used.<br>
**Using more threads increases the system memory usage**.<br>
Use *'--threads \<number\>'* to specify a concrete number of threads.
The output ist then later printed out line-by-line in the console.<br>

Use *'-s \<node id\>'* to specify the node from which the one-to-all dijkstra should be run.<br>
The target node can be either given as a flag using *'-t \<node id>'* or entered later in the console.
