#!/bin/bash

RED_COLOR='\033[0;31m'
GREEN_COLOR='\033[0;32m'
PURPLE_COLOR='\033[0;35m'
DEFAULT_COLOR='\033[0m'

function log() {
  if [[ $2 = "error" ]]; then
    printf "${PURPLE_COLOR}>> ${RED_COLOR} (!) $1${DEFAULT_COLOR}\n\n"
  else
    printf "${PURPLE_COLOR}>> ${GREEN_COLOR} $1${DEFAULT_COLOR}\n\n"
  fi
}

# Commits the current state of "develop" branch
# builds and push.
# Then checkouts to "master" branch and pushes the build
commit_and_publish () {
  echo "Commit message:"
  read COMMIT_MESSAGE

  log "Building estebanborai/estebanborai.github.io"
  npm run build

  log "Copying files to external `bundle` directory"
  mkdir  ~/esteban-borai-github-io-bundle-temporal-dir/
  cp -R ./dist/** ~/esteban-borai-github-io-bundle-temporal-dir/

  log "Checking out the current branch"
  git add .
  git commit -m "[🤖] $COMMIT_MESSAGE"
  git push origin develop

  log "Copying new bundle files"
  git checkout master
  # yes | cp -rf ~/esteban-borai-github-io-bundle-temporal-dir/** ./
  cp -rf ~/esteban-borai-github-io-bundle-temporal-dir/** ./
  
  log "Publshing to GitHub Pages"
  git add .
  git commit -m "[🤖] $COMMIT_MESSAGE"
  git push origin master
  git checkout develop

  log "Removing temporal directory \"~/esteban-borai-github-io-bundle-temporal-dir/\""
  rm -rf ~/esteban-borai-github-io-bundle-temporal-dir/

  log "Build finished"
}

commit_and_publish
