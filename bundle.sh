#!/bin/bash

RED_COLOR='\033[0;31m'
GREEN_COLOR='\033[0;32m'
PURPLE_COLOR='\033[0;35m'
DEFAULT_COLOR='\033[0m'

function log() {
	if [[ $2 = "error" ]]; then
		printf "${PURPLE_COLOR}活動 ${RED_COLOR} (!) $1${DEFAULT_COLOR}\n\n"
	else
		printf "${PURPLE_COLOR}活動 ${GREEN_COLOR} $1${DEFAULT_COLOR}\n\n"
	fi
}

commit_and_publish () {
	echo "Commit message:"
	read COMMIT_MESSAGE

	log "Building estebanborai.github.io"
	yarn && yarn build

	log "Copying files to external `bundle` directory"
	mkdir -p ~/bundle-temporal-dir/data
	cp -R ./bundle/** ~/bundle-temporal-dir/
	cp -R ./data/** ~/bundle-temporal-dir/data/

	log "Checking out the current branch"
	git add .
	git commit -m "[🤖] $COMMIT_MESSAGE"
	git push origin develop

	log "Copying new bundle files"
	git checkout master
	yes | cp -rf ~/bundle-temporal-dir/** ./
	rm -rf ~/bundle-temporal-dir/

	log "Publshing to GitHub Pages"
	git add .
	git commit -m "[🤖] $COMMIT_MESSAGE"
	git push origin master
	git checkout develop

	log "Build finished"
}

commit_and_publish