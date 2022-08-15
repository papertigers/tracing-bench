out_stacks := "out.user_stacks"
svg := "tracing-bench.svg"
svg_registry := "tracing-bench.registry.svg"

@_default:
  just --list

_build:
  cargo build --release --no-default-features

_build-registry:
  cargo build --release

_hyperfine:
  hyperfine --shell=none --warmup 10 './target/release/tracing-bench -c 1000 -t 10'
	
# run hyperfine without dynamic log filters
test: _build _hyperfine

# run hyperfine with dynamic log filters
test-registry: _build-registry _hyperfine

_clean-dtrace file:
  pfexec rm -f {{file}}

_dtrace output: (_clean-dtrace output)
  pfexec dtrace -x ustackframes=100 -n \
      'profile-97 /pid == $target && arg1/ {@[ustack()] = count()}' \
      -c './target/release/tracing-bench -c 10000000 -t 2' \
      -o {{output}}

_clean-svg file:
  rm -f {{file}}

_inferno stacks output: (_clean-svg output) (_dtrace "out.user_stacks")
  demangle < {{stacks}} \
    | inferno-collapse-dtrace \
    | inferno-flamegraph > {{output}}

# generate a flamegraph without dynamic log filters
flamegraph: _build (_inferno out_stacks svg)

# generate a flamegraph with dynamic log filters
flamegraph-registry: _build-registry (_inferno out_stacks svg_registry)
