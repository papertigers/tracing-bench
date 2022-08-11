out_stacks := "out.user_stacks"
svg := "tracing-bench.svg"
svg_registry := "tracing-bench.registry.svg"

build:
  cargo build --release --no-default-features

build-registry:
  cargo build --release

hyperfine:
  hyperfine --shell=none --warmup 10 './target/release/tracing-bench -c 1000 -t 10'
	
test: build
  just hyperfine

test-registry: build-registry
  just hyperfine

clean-dtrace:
  pfexec rm -f {{out_stacks}}

dtrace: clean-dtrace
  pfexec dtrace -x ustackframes=100 -n \
      'profile-97 /pid == $target && arg1/ {@[ustack()] = count()}' \
      -c './target/release/tracing-bench -c 10000000 -t 2' \
      -o {{out_stacks}}

inferno OUTPUT:
  rm -f {{svg}}
  just dtrace
  demangle < {{out_stacks}} \
    | inferno-collapse-dtrace \
    | inferno-flamegraph > {{OUTPUT}}

flamegraph: build
  just inferno {{svg}}


flamegraph-registry: build-registry
  just inferno {{svg_registry}}


