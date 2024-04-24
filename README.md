NETWORK FLOW SIMULATION

PROBLEM

As input, you are given a list of `m` connections and the latency between `n` nodes. Imagine a broadcast system in this network. At any turn, the node `i` will send a message to all of its neighbors and they also send the message to their neighbors as soon as they get it (assume zero latency in this step).

We measure the total time it takes for the message to reach every node in the network that it can reach. For each node `i` you should print two numbers in the output:
- the total propagation time until every node was met
- the number of nodes the message reached

Example input:
```
3 4
0 1 200
1 2 350
0 2 500
1 2 600
```


SOLUTION

For each node `i` run Dijkstra's shortest path algorithm (latencies of a connection is a weight of a respective edge of a weighted graph modeled with the adjacency list). After traversal starting from node `i` is completed, all reachable nodes are going to be traversed (unreachable nodes can only occur in the event of network partition, i.e. disjointed graph) and the shortest path (which will also be the lowest latency) will be known.


EXAMPLE

```
$ cargo run < example.txt
<snip>
node 0: time 500, nodes 2
node 1: time 350, nodes 2
node 2: time 500, nodes 2
```


SHORTCUTS

- no error management (panic if anything goes wrong)
- no performance optimisations (parallel, GPU etc)
