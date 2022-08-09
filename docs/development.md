# Development

## Linting

Execute MegaLinter locally:

```s
./bin/ml.sh
```

## Testing

Run all tests:

```s
cargo test
```

Run only one test:

```s
cargo test test_lambda
```

Run tests with `nextest`:

```s
cargo nextest run
```

Generate a coverage report:

```s
cargo llvm-cov nextest
```

- [Install nextest](https://nexte.st/book/pre-built-binaries.html).
- [Generate test coverage](https://nexte.st/book/test-coverage.html).
