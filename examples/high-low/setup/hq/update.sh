git remote update -p
git merge --allow-unrelated-histories -m "hq: merge store A" store-a/main
git merge --allow-unrelated-histories -m "hq: merge store B" store-b/main
