#!/bin/bash

if [ -f /usr/bin/pikman-update-manager ]
then
	pikman-update-manager
else
	/usr/lib/pika/pika-welcome/scripts/software-manager.sh
fi
