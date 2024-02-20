export PATH := $(PWD):$(PATH)

all:
	true

install:
	mkdir -p $(DESTDIR)/usr/bin/
	cargo fetch
	cargo build --release
	cp -vf target/release/pika-first-setup-gtk4 $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/pika-first-setup-gtk4
	mkdir -p $(DESTDIR)/usr/share/glib-2.0/schemas/
	mkdir -p $(DESTDIR)/usr/lib/pika/pika-first-setup-gtk4/scripts/
	cp -rvf data/scripts/*.sh $(DESTDIR)/usr/lib/pika/pika-first-setup-gtk4/scripts/
	chmod 755 $(DESTDIR)/usr/lib/pika/pika-first-setup-gtk4/scripts/*.sh
	cp data/com.github.pikaos-linux.pikafirstsetup.gschema.xml $(DESTDIR)/usr/share/glib-2.0/schemas/
	mkdir -p $(DESTDIR)/usr/share/applications
	cp -vf data/com.github.pikaos-linux.pikafirstsetup.desktop  $(DESTDIR)/usr/share/applications/
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/com.github.pikaos-linux.pikafirstsetup.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	#makepot $(DESTDIR)/usr/share/locale
