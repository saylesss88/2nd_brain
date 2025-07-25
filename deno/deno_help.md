---
id: deno_help
aliases: []
tags: []
---

❯ deno --help
A modern JavaScript and TypeScript runtime

Docs: https://deno.land/manual@v1.45.5
Standard Library: https://jsr.io/@std
Modules: https://jsr.io/ https://deno.land/x/
Bugs: https://github.com/denoland/deno/issues

To start the REPL:

  deno

To execute a script:

  deno run https://examples.deno.land/hello-world.ts

To evaluate code in the shell:

  deno eval "console.log(30933 + 404)"


Usage: deno [OPTIONS] [COMMAND]

Commands:
  run          Run a JavaScript or TypeScript program
  serve        Run a server
  add          Add dependencies
  bench        Run benchmarks
  bundle       Bundle module and dependencies into single file
  cache        Cache the dependencies
  check        Type-check the dependencies
  compile      Compile the script into a self contained executable
  completions  Generate shell completions
  coverage     Print coverage reports
  doc          Show documentation for a module
  eval         Eval script
  fmt          Format source files
  init         Initialize a new project
  info         Show info about cache or info related to source file
  install      Install script as an executable
  jupyter      Deno kernel for Jupyter notebooks
  uninstall    Uninstall a script previously installed with deno install
  lsp          Start the language server
  lint         Lint source files
  publish      Publish the current working directory's package or workspace
  repl         Read Eval Print Loop
  task         Run a task defined in the configuration file
  test         Run tests
  types        Print runtime TypeScript declarations
  upgrade      Upgrade deno executable to given version
  help         Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet
          Suppress diagnostic output

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

      --unstable
          Enable unstable features and APIs

      --unstable-bare-node-builtins
          Enable unstable bare node builtins feature
          
          [env: DENO_UNSTABLE_BARE_NODE_BUILTINS=]

      --unstable-byonm
          Enable unstable 'bring your own node_modules' feature
          
          [env: DENO_UNSTABLE_BYONM=]

      --unstable-sloppy-imports
          Enable unstable resolving of specifiers by extension probing, .js to .ts, and directory probing.
          
          [env: DENO_UNSTABLE_SLOPPY_IMPORTS=]

      --unstable-broadcast-channel
          Enable unstable `BroadcastChannel` API

      --unstable-cron
          Enable unstable Deno.cron API

      --unstable-ffi
          Enable unstable FFI APIs

      --unstable-fs
          Enable unstable file system APIs

      --unstable-http
          Enable unstable HTTP APIs

      --unstable-kv
          Enable unstable Key-Value store APIs

      --unstable-net
          Enable unstable net APIs

      --unstable-process
          Enable unstable process APIs

      --unstable-temporal
          Enable unstable Temporal API

      --unstable-unsafe-proto
          Enable unsafe __proto__ support. This is a security risk.

      --unstable-webgpu
          Enable unstable `WebGPU` API

      --unstable-worker-options
          Enable unstable Web Worker APIs

Environment variables:
    DENO_AUTH_TOKENS     A semi-colon separated list of bearer tokens and
                         hostnames to use when fetching remote modules from
                         private repositories
                         (e.g. "abcde12345@deno.land;54321edcba@github.com")

    DENO_FUTURE          Set to "1" to enable APIs that will take effect in
                         Deno 2

    DENO_CERT            Load certificate authorities from PEM encoded file

    DENO_DIR             Set the cache directory

    DENO_INSTALL_ROOT    Set deno install's output directory
                         (defaults to $HOME/.deno/bin)

    DENO_JOBS            Number of parallel workers used for the --parallel
                         flag with the test subcommand. Defaults to number
                         of available CPUs.

    DENO_REPL_HISTORY    Set REPL history file path
                         History file is disabled when the value is empty
                         (defaults to $DENO_DIR/deno_history.txt)

    DENO_NO_PACKAGE_JSON Disables auto-resolution of package.json

    DENO_NO_PROMPT       Set to disable permission prompts on access
                         (alternative to passing --no-prompt on invocation)

    DENO_NO_UPDATE_CHECK Set to disable checking if a newer Deno version is
                         available

    DENO_TLS_CA_STORE    Comma-separated list of order dependent certificate
                         stores. Possible values: "system", "mozilla".
                         Defaults to "mozilla".

    DENO_V8_FLAGS        Set V8 command line options

    DENO_WEBGPU_TRACE    Directory to use for wgpu traces

    HTTP_PROXY           Proxy address for HTTP requests
                         (module downloads, fetch)

    HTTPS_PROXY          Proxy address for HTTPS requests
                         (module downloads, fetch)

    NO_COLOR             Set to disable color

    NO_PROXY             Comma-separated list of hosts which do not use a proxy
                         (module downloads, fetch)

    NPM_CONFIG_REGISTRY  URL to use for the npm registry.
