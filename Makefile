all:
	true

install:
	mkdir -p $(DESTDIR)/usr/bin/
	cargo fetch
	cargo build --release
	cp -vf target/release/gtk4-rs-adw-project-template $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/gtk4-rs-adw-project-template
	mkdir -p $(DESTDIR)/usr/share/glib-2.0/schemas/
	cp data/org.cosmicfusion.example.xml $(DESTDIR)/usr/share/glib-2.0/schemas/
	mkdir -p $(DESTDIR)/usr/share/applications
	cp -vf data/org.cosmicfusion.example.desktop  $(DESTDIR)/usr/share/applications/
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/org.cosmicfusion.example.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
