#!bin/bash
echo "Building estebanborai.github.io"
yarn && yarn build

echo "Copying files to external `bundle` directory"
mkdir -p ~/bundle-temporal-dir/
cp -R ./bundle/** ~/bundle-temporal-dir/

echo "Checking out the current branch"
git add .
git commit -m "[Script] Publishing a new version"
git push origin develop

echo "Copying new bundle files"
git checkout master
cp -R ~/bundle-temporal-dir/** ./
rm -rf ~/bundle-temporal-dir/

echo "Publshing to GitHub Pages"
git add .
git commit -m "[Script] Published"
git push origin master
git checkout develop

echo "Build finished"
