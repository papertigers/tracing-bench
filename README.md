# tracing-bench

This is a small program to reproduce a performance issue we noticed when
dynamically changing tracing log layers via a `Registry`.

[Without dynamic layers](https://lightsandshapes.com/tracing-bench.svg)

[With dynamic layers](https://lightsandshapes.com/tracing-bench.registry.svg)

## Requirements

- inferno
- hyperfine
- just
- assumes an OS with DTrace at the moment


## To run

Flamegraph of program without using dynamic layers:
```
just flamegraph
```

Hyperfine output without dynamic layers:
```
just test
```

Flamegraph of program using dynamic layers:
```
just flamegraph-registry
```

Hyperfine output with dynamic layers:
```
just test-registry
```
