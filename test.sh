for f in src/*
do
        echo "Testing $f:"
        rustc $f 2>&1 | grep "internal compiler error" || exit 1
done
