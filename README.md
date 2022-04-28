# Logagg

This is a simple log aggregator service. The service can be ran by anyone to receive streamed logs from multiple sources (mainly VMs). [tailstream](https://github.com/threefoldtech/tailstream) is the tool which used in our VMs.

## Configuration
Configuration is passed via `--cfg` argument which accepts a `path` to a `yaml` file.

The `yaml` file has the following structure.
```yaml
listen: <listen address>
output:
  - kind: <output type i.e. file>
    config: < i.e. /where/to/keep/thelogs/ for a *file* kind>
  - kind: ...
    config: ...
```

## Usage
```
logagg --cfg /path/to/<config>.yaml
```
