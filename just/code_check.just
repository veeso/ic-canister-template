# Formats the code
[group('code_check')]
fmt args="":
  cargo fmt --all {{args}}


# Formats the code using the nightly version of rustfmt
[group('code_check')]
fmt_nightly args="":
  cargo +nightly fmt --all {{args}}


# Runs clippy on the code
[group('code_check')]
clippy args="":
  export PROTOC_INCLUDE=${PWD}/proto
  cargo clippy --all-features --all-targets {{args}}


# Runs fmt and clippy on the code. It fails if any of the checks fail
[group('code_check')]
check_code:
  just fmt_nightly "--check"
  just clippy "-- -D warnings"
