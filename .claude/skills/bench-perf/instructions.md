# Performance Benchmark Skill

## Purpose
Validates that Patina meets its performance targets defined in README.md and CLAUDE.md.

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Cold start time | < 50ms | Time to show help text |
| Memory footprint (idle) | < 20 MB | RSS after opening empty doc |
| Binary size | < 15 MB | Release binary with strip |

## What This Skill Does

1. Builds optimized release binary
2. Measures startup time
3. Checks binary size
4. Reports results against targets

## Steps to Execute

### 1. Build Release Binary
```bash
cargo build --release -p patina
```

### 2. Measure Binary Size
```bash
ls -lh target/release/patina
```

Check that size is < 15MB. If over, report regression.

### 3. Measure Cold Startup Time

**On macOS/Linux:**
```bash
# Clear disk cache (macOS)
sync && sudo purge

# Time startup
time target/release/patina --help
```

**On Linux:**
```bash
# Clear disk cache
sync && echo 3 | sudo tee /proc/sys/vm/drop_caches

# Time startup
time target/release/patina --help
```

Extract the "real" time from output. Should be < 50ms.

### 4. Measure Memory Footprint (Optional)

**On macOS:**
```bash
# Launch patina with empty doc
target/release/patina /tmp/empty.md &
PID=$!

# Get RSS memory
ps -o rss= -p $PID

# Clean up
kill $PID
```

**On Linux:**
```bash
# Launch and measure
/usr/bin/time -v target/release/patina --help 2>&1 | grep "Maximum resident set size"
```

Should be < 20MB.

## Success Criteria

All metrics must be within targets:
- ✅ Binary < 15MB
- ✅ Startup < 50ms
- ✅ Memory < 20MB

## On Failure

If any metric exceeds target:
1. **Binary size too large:**
   - Check for debug symbols: `strip target/release/patina`
   - Review dependencies: `cargo tree`
   - Look for accidentally included resources

2. **Startup too slow:**
   - Profile with `cargo flamegraph`
   - Check for unnecessary initialization
   - Review dependency lazy_static/once_cell usage

3. **Memory too high:**
   - Profile with `heaptrack` or `valgrind --tool=massif`
   - Check for memory leaks
   - Review buffer allocations

## Notes

- Run benchmarks multiple times and take average
- Close other applications to reduce noise
- Benchmark on target hardware (not inside VM if possible)
- Compare results to previous commits to detect regressions
