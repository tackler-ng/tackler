cp "$2" txns/store-b/

git add txns/store-b/sales-$1.txn
git commit -m 'store-b: sales txn $1'
git push
