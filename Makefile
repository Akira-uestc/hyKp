build:
	cargo build

install:build
	mkdir -p /home/$(USER)/.config/hykp
	sudo cp ./target/debug/hykp /usr/local/bin/
	cp ./hykp.service /home/$(USER)/.config/systemd/user/
	systemctl --user enable hykp.service
	systemctl --user start hykp.service

uninstall:
	sudo rm /usr/local/bin/hykp
	rm -f /home/$(USER)/.config/systemd/user/hykp.service
	systemctl --user disable hykp.service
	systemctl --user stop hykp.service

