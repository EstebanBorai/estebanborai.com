#!/bin/bash
commit_and_publish () {
	echo "Commit message:"
	read COMMIT_MESSAGE

	echo "Building estebanborai.github.io"
	yarn && yarn build

	echo "Copying files to external `bundle` directory"
	mkdir -p ~/bundle-temporal-dir/data
	cp -R ./bundle/** ~/bundle-temporal-dir/
	cp -R ./data/** ~/bundle-temporal-dir/data/

	echo "Checking out the current branch"
	git add .
	git commit -m "[Script] $COMMIT_MESSAGE"
	git push origin develop

	echo "Copying new bundle files"
	git checkout master
	yes | cp -rf ~/bundle-temporal-dir/** ./
	rm -rf ~/bundle-temporal-dir/

	echo "Publshing to GitHub Pages"
	git add .
	git commit -m "[Script] $COMMIT_MESSAGE"
	git push origin master
	git checkout develop

	echo "Build finished"
}

commit_and_publish