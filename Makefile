INSTALL_MODE="GLOBAL"
BIN_LOCATION=/usr/local/bin/
USER_BIN_LOCATION=~/.local/bin/

LOG:="\033[92m●\033[0m \033[92m[INFO]\033[0m"
WARNING:="\033[93m●\033 [0m\033[93m[WARNING]\033[0m"
ERROR:="\033[91m●\033[0m \033[91m[ERROR]\033[0m"
MAKETAG:="\033[1m[make]\033[0m"

build:
	@echo -e $(MAKETAG) $(LOG) "Building executable"
	cargo build --release

install: build
	@echo -e $(MAKETAG) $(LOG) "Installing in mode $(INSTALL_MODE)"
	@if [ $(INSTALL_MODE) == "GLOBAL" ]; then \
		echo -e $(MAKETAG) $(LOG) "Installing to folder $(BIN_LOCATION)"; \
		sudo cp ./target/release/fetch $(BIN_LOCATION)fetch; \
		if [ $$? -ne 0 ]; then \
			echo -e $(MAKETAG) $(ERROR) "Install unsuccessful!"; \
			exit 1; \
		fi; \
		echo -e $(MAKETAG) $(LOG) "Installation successful!"; \
	elif [ $(INSTALL_MODE) == "USER" ]; then \
		echo -e $(MAKETAG) $(LOG) "Installing to folder $(USER_BIN_LOCATION)"; \
		cp ./target/release/fetch $(USER_BIN_LOCATION)fetch; \
		if [ $$? -ne 0 ]; then \
			echo -e $(MAKETAG) $(ERROR) "Install unsuccessful!"; \
			exit 1; \
		fi; \
		echo -e $(MAKETAG) $(LOG) "Installation successful!"; \
	fi
