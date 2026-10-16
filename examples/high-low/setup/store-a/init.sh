git init --bare store-a.git
git clone store-a.git/

cd store-a
git config user.name "ACME Store A"
git config user.email "store-a@acme"
git config commit.gpgSign no
git commit -m init --allow-empty

mkdir conf
cp "$@" conf/
mkdir -p txns/store-a

git add conf
git commit -m "Store A conf"
git push
