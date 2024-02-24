export PATH := $(PWD):$(PATH)

all:
	true

install:
	mkdir -p $(DESTDIR)/usr/bin/
	cargo fetch
	cargo build --release
	cp -vf target/release/pika-welcome $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/pika-welcome
	cp -vf data/bin/pika-welcome $(DESTDIR)/usr/bin/pika-welcome-autostart
	chmod 755 $(DESTDIR)/usr/bin/pika-welcome-autostart
	mkdir -p $(DESTDIR)/usr/share/glib-2.0/schemas/
	mkdir -p $(DESTDIR)/usr/share/
	mkdir -p $(DESTDIR)/usr/lib/pika/pika-welcome/scripts/
	cp -rvf data/scripts/*.sh $(DESTDIR)/usr/lib/pika/pika-welcome/scripts/
	chmod 755 $(DESTDIR)/usr/lib/pika/pika-welcome/scripts/*.sh
	cp data/com.github.pikaos-linux.pikawelcome.gschema.xml $(DESTDIR)/usr/share/glib-2.0/schemas/
	mkdir -p $(DESTDIR)/usr/share/applications
	cp -vf data/com.github.pikaos-linux.pikawelcome.desktop  $(DESTDIR)/usr/share/applications/
	mkdir -p $(DESTDIR)/etc/xdg/autostart
	cp -vf data/pika-welcome-autostart.desktop  $(DESTDIR)/etc/xdg/autostart/
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/com.github.pikaos-linux.pikawelcome.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	cp -vf data/icons $(DESTDIR)/usr/share/
	#makepot $(DESTDIR)/usr/share/locale
