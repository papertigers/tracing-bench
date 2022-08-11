out_stacks := "out.user_stacks"
stacks_folded := "stacks.folded"
svg := "tracing-bench.svg"
svg_registry := "tracing-bench.registry.svg"

build:
  cargo build --release --no-default-features

build-registry:
  cargo build --release
	
test: build
  hyperfine --shell=none --warmup 10 './target/release/tracing-bench -c 1000 -t 10'

test-registry: build-registry
  hyperfine --shell=none --warmup 10 './target/release/tracing-bench -c 1000 -t 10'

clean-dtrace:
  pfexec rm -f {{out_stacks}}
  rm -f {{stacks_folded}}

dtrace:
  pfexec dtrace -x ustackframes=100 -n \
      'profile-97 /pid == $target && arg1/ {@[ustack()] = count()}' \
      -c './target/release/tracing-bench -c 10000000 -t 2' \
      -o {{out_stacks}}

fold:
  cat {{out_stacks}} | demangle | inferno-collapse-dtrace > {{stacks_folded}}

flamegraph: build
  rm -f {{svg}}
  just dtrace
  just fold
  cat {{stacks_folded}} | inferno-flamegraph > {{svg}}


flamegraph-registry: build-registry
  rm -f {{svg_registry}}
  just dtrace
  just fold
  cat {{stacks_folded}} | inferno-flamegraph > {{svg_registry}}


