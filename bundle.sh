#!bin/bash
echo "Building estebanborai.github.io"

yarn build

echo "Copying files to external `bundle` directory"
mkdir -p ../bundle/
cp -R ./bundle/** ../bundle/

echo "Checking out the current branch"
git add .
git commit -m "Bundle"
git push origin develop
git checkout master

echo "Copying new bundle files"
cp -R ../bundle/** ./
rm -rf ../bundle/

echo "Publshing to GitHub Pages"
git add .
git commit -m "Published"
git push origin master
git checkout develop

echo "Build finished"
