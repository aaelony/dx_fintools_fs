

help: ## Show this help message
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z0-9_-]+:.*##/ { printf "%-30s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

check:  ## cargo check
		cargo check

serve-web: ## dx serve --platform web
	dx serve --platform web

bundle-release-web: ## dx bundle --platform web --release
	dx bundle --platform web --release
