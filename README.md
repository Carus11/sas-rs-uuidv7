# SAS-RS-UUIDv7

A Rust library for wrapping the uuid crate function for generating UUIDv7 values that can be called from SAS.

## Overview

This library provides a simple function to generate UUIDv7 values that can be called from SAS using PROC PROTO.

## Building

### Standard Build
```bash
cargo build --release
```

The compiled library will be available at `target/release/libsas_rs_uuidv7.so`.

### Building for RHEL UBI8 Compatibility

To build a version compatible with RHEL UBI8 (which has an older glibc), use the included Docker build script:

```bash
./build_ubi8.sh
```

This script:
1. Creates a Docker container based on UBI8
2. Builds the library inside the container with the exact glibc version from UBI8
3. Extracts the compiled library to the `./build/` directory

Requirements for the UBI8 build:
- Docker installed and running
- Internet access for pulling the UBI8 image

The UBI8-compatible library will be available at `./build/libsas_rs_uuidv7.so`.

## Usage in SAS

Configure the Compute servers restriction options in Environment Manager `sas.compute.server:restricted_options`
```
-PROTOLIBS=("/path/to/libs")
```

```sas
/* Load the Rust library with PROC PROTO */
options cmplib=work.myfuncs;

proc proto package=work.myfuncs.uuid 
  label='Rust UUID v7 Generator';
  /* Link to the compiled library */
  link "/path/to/libsas_rs_uuidv7.so";
  char* uuidv7();
run;

/* Create a wrapper function */
proc fcmp outlib=work.myfuncs.new_uuid;
  function new_uuidv7() $36;
    uuid = uuidv7();
    return(uuid);
  endsub;
run;

/* Call the wrapper function */
data _null_;
  length uuid $ 36;
  uuid = new_uuidv7();
  put uuid=;
run;
```

## Example Output

```
uuid=01893e56-3d48-7c40-8e71-9a834a7e9cee
```

## Requirements

- Rust 1.70+ (for UUIDv7 support)
- SAS with PROC PROTO support
- Docker or cli compatible alternative
