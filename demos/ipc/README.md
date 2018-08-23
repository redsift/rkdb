IPC Example
===========

This example will link the rkdb library and create a binary `ipc`.

To run the demo, start a local q process serving on port 12001 then execute:

```
q -p 12001
target/release/ipc
```

The program will contact the q process, run a query, and print the result.
