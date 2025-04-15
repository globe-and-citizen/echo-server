# echo-server
Practice repo for working with Rust, crypto, and WASM 

### Run options:

- Make using tmux:
  - Install [`tmux`](https://github.com/tmux/tmux/wiki/Installing)
  - ```bash
    make tmux
    ```

- Make using npm concurrently:
  - Install concurrency `npm install -g concurrently`
  - ```bash
    make
    ```

- Manual:

Frontend
```bash
cd frontend
npm run dev
```

Proxy
```bash
cd proxy
cargo run
```