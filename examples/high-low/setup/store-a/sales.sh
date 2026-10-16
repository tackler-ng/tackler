cp "$2" txns/store-a/

git add txns/store-a/sales-$1.txn
git commit -m "store-a: sales txn $1"
git push
