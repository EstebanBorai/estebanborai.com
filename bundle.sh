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

cp -R ../bundle/** ./
rm -rf ../bundle/

echo "Build finished"
