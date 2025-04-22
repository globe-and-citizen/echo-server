# echo-server
Practice repo for working with Rust, crypto, and WASM 

### Run options:

- Make using tmux:
  - Install [`tmux`](https://github.com/tmux/tmux/wiki/Installing)
  - ```bash
    # run with default configuration
    make tmux
    
    # run with custom configuration (.toml file)
    make tmux CONFIG_PATH=[path to config]
    ```

- Make using npm concurrently:
  - Install concurrency `npm install -g concurrently`
  - ```bash
    # run with default configuration
    make
    
    # run with custom configuration (.toml file)
    make CONFIG_PATH=[path to config]
    ```

- Manual:

  - Frontend:
    ```bash
    cd frontend
    npm install # for the first time
    npm run dev
    ```

  - Proxy:
    ```bash
    cd proxy
    cargo build # for the first time
    cargo run --CONFIG_PATH=[path to config] # --CONFIG_PATH is optional
    ```
