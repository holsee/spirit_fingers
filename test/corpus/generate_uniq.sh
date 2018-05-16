for ((i = 0; i < $1; i++)); do
  base64 /dev/urandom | head -c 100000 > "s-$i.txt";
done
