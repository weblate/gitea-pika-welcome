#! /usr/bin/bash
export LANG=C
DISPLAY_CHECK=$(echo $DISPLAY)
WHO=$(whoami)

### INTERNET CHECK ###
INTERNET="no"

internet_check() {
      # Check for internet connection
      wget -q --spider http://google.com
      if [ $? -eq 0 ]; then
          export INTERNET="yes"
      fi
}

### VIDEO CODEC CHECK ####
INSTALLED="no"

package_check() {
      # Check if codecs are already installed
      /usr/lib/pika/pika-welcome/scripts/check-pkg.sh pika-codecs-meta
      if [ $? -eq 0 ]; then
          export INSTALLED="yes"
      fi
}

SUCCESSFUL="no"

codec_install() {
      (
        PASSWORD=$(zenity --password --title='Password Authentication')
	echo "15"; sleep 1
        echo "# Updating repository information"
	# refresh repo metadata
	echo $PASSWORD | sudo -S apt update &>/tmp/codeccheck.log
	# update repos so that we can see any new repo package changes
	echo $PASSWORD | sudo -S apt install -y pika-sources &>>/tmp/codeccheck.log
	# refresh repo data again.
	echo $PASSWORD | sudo -S apt update &>>/tmp/codeccheck.log
        echo "50"; sleep 1
	echo "# Installing codec meta package, includes hardware decoding and ffmpeg stuff"
	echo $PASSWORD | sudo -S apt install -y pika-codecs-meta &>>/tmp/codeccheck.log
        echo "100"; sleep 1
      ) | zenity --title "Video Playback and Encoding enablement" --progress --width=600 --no-cancel --auto-close --percentage=0
}

codec_ask() {
		            	if zenity --question \
		              	--title="Video Playback and Encoding enablement" \
		              	--width=600 \
		              	--text="`printf "Due to U.S. patent laws we are not able to include some important video playback and
encoding packages on the PikaOS installation media, -HOWEVER- these are freely
available to download and install with your consent, which we are asking for now! \n

Please note that without these packages installed, video playback in some games, browsers,
and media players will not work correctly. Additionally, without these packages you will
be unable to use video encoding in OBS studio and Blender.\n

Would you like to install the required video playback and encoding packages now to resolve
the issue? (strongly recommended) \n"`"
				then
					codec_install && export SUCCESSFUL="yes"
		              		if [[ $SUCCESSFUL == "yes" ]]; then
								zenity --info --title='Complete!' --text="$(printf "Installation Complete! \nYou may want to reboot for changes to take effect.\n Please do it ASAP")"
							else
								zenity --error --title='Failed!' --text='Failed to install codecs!'
							fi
				fi
}

package_check
internet_check

### Start Program ###

if [[ $INSTALLED == "no" ]]; then
	if [[ $INTERNET == "yes" ]]; then
		if [[ $DISPLAY_CHECK ]] && [[ $WHO != "pikaos" ]] && [[ $WHO != "gnome-initial-setup" ]]; then
		  # sometimes if this tries to run too early when the session is still starting KDE will freeze
		  sleep 5
		  codec_ask
		fi
	else
	zenity --error --title='Failed!' --text='No Internet Connection!'
	fi
else
	zenity --error --title='No Codec Change Required!' --text='All required Codecs are already installed!'
fi
