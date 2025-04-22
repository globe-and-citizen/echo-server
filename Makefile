CONFIG_ARG := $(if $(CONFIG_PATH),CONFIG_PATH=$(CONFIG_PATH) )
RUN_PROXY := "cd proxy && $(CONFIG_ARG) cargo run"
RUN_FRONTEND := "cd frontend && npm run dev"

run tmux:
	tmux new-session -d -s echo-server $(RUN_FRONTEND) \; \
	split-window -h $(RUN_PROXY) \; \
	attach

run:
	concurrently $(RUN_FRONTEND) $(RUN_PROXY)